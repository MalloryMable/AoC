import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;

public class AoC_2015_10 {
    public static void main(String[] args) throws FileNotFoundException {
        Scanner scanner = args.length != 0 ? new Scanner(args[0]) : new Scanner(new File("file.txt"));
        String input = scanner.nextLine();
        for(int j = 0; j < 50; j ++){
            StringBuilder tempString = new StringBuilder();
            for (int i = 0; i < input.length(); i++) {
                char local = input.charAt(i);
                int index = i;
                do {
                    i++;
                }  while (i < input.length() && local == input.charAt(i));
                index = i - index;
                i--;
                tempString.append(index).append(local);
            }
            input = tempString.toString();
            if(j!= 0 && j%40.0 == 0) {
                System.out.println(input.length());
            }
        }
        System.out.println(input.length());
    }
}
