import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;
import java.util.regex.Pattern;

public class AoC_2015_5_1 {
    public static void main(String[] args) throws FileNotFoundException {

        File file = args.length != 0 ? new File(args[0]) : new File("file.txt");
        Scanner scanner = new Scanner(file);
        int count = 0;

        while(scanner.hasNext()) {
            String line = scanner.nextLine();
            Pattern threeVowels = Pattern.compile("[aeiou].*[aeiou].*[aeiou]", Pattern.CASE_INSENSITIVE);
            Pattern doubleLetter = Pattern.compile("(.)\\1", Pattern.CASE_INSENSITIVE);
            Pattern bad = Pattern.compile("ab|cd|pq|xy", Pattern.CASE_INSENSITIVE);

            if(threeVowels.matcher(line).find() && doubleLetter.matcher(line).find() && !bad.matcher(line).find()) {
                count++;
            }
        }
        scanner.close();
        System.out.printf("Number of nice kids: %d", count);
    }
}
