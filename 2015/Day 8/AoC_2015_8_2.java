import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;

public class AoC_2015_8_2 {
    public static void main(String[] args) throws FileNotFoundException {
        File file = args.length != 0 ? new File(args[0]) : new File("file.txt");
        Scanner scanner = new Scanner(file);
        int goodSum = 0, escapedSum = 0;
        while(scanner.hasNext()){
            String line = scanner.next();
            escapedSum += line.length();
            line = "    " + line;

            for(int letter = 0; letter < line.length(); letter++){
                if(line.charAt(letter) == '\\'){
                    if(line.charAt(letter + 1) == 'x') {
                        line = line.substring(0,letter) + "     " + line.substring(letter + 4);

                    } else {
                        line = line.substring(0,letter) + "    " + line.substring(letter + 2);

                    }
                    letter++;
                }
            }
            goodSum += line.length();
        }
        scanner.close();

        System.out.printf("Good sum %d - %d: %d",goodSum, escapedSum, goodSum - escapedSum);
    }
}
