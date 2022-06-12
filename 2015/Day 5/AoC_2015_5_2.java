import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;
import java.util.regex.Pattern;

public class AoC_2015_5_2 {
    public static void main(String[] args) throws FileNotFoundException {

        File file = args.length != 0 ? new File(args[0]) : new File("file.txt");
        Scanner scanner = new Scanner(file);
        int count = 0;

        while(scanner.hasNext()) {
            String line = scanner.nextLine();
            Pattern doubleLetter = Pattern.compile("(..).*\\1", Pattern.CASE_INSENSITIVE);
            Pattern split = Pattern.compile("(.).\\1", Pattern.CASE_INSENSITIVE);

            if(doubleLetter.matcher(line).find() && split.matcher(line).find()) {
                count++;
            }
        }
        scanner.close();
        System.out.printf("Number of nice kids: %d", count);
    }
}
