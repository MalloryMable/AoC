import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;

public class AoC_2015_1 {

    public static void main(String[] args) throws FileNotFoundException {

        File file = args.length != 0 ? new File(args[0]) : new File("file.txt");
        Scanner scanner = new Scanner(file);
        int count = 0;
        while(scanner.hasNext()) {
            String line = scanner.nextLine();
            for(int x = 0; x < line.length(); x++){
                if ((line.charAt(x) == '(')) {
                    count++;
                } else {
                    count--;
                }
            }
        }
        scanner.close();
        System.out.printf("Santa should go to floor %s.", count);
    }
}
