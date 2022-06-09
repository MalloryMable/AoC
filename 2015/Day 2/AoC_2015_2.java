import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;

public class AoC_2015_2 {
    public static void main(String[] args) throws FileNotFoundException {

        File file = args.length != 0 ? new File(args[0]) : new File("file.txt");
        Scanner scanner = new Scanner(file);
        int min, presentSum = 0, ribbonSum = 0;
        while(scanner.hasNextLine()) {
            // Length = d[0], Width = d[1], Height = d[2]
            String[] dimension = scanner.nextLine().split("x");
            int wrap,
                    length = Integer.parseInt(dimension[0]),
                    width = Integer.parseInt(dimension[1]),
                    height = Integer.parseInt(dimension[2]);

            int max = Math.max(Math.max(length, width), height);

            //By ignoring the max value we find the min sides
            if(length == max) {
                min = width * height;
                wrap = 2 * (width + height);
            } else if (width == max) {
                min = length * height;
                wrap = 2 * (length + height);
            } else {
                min = length * width;
                wrap = 2 * (length + width);
            }

            //2*l*w + 2*w*h + 2*h*l = 2(w(l+h) + h*l)
            presentSum += 2 * (width * (length + height) + (height * length)) + min;
            //min1 + min1 + min2 + min2 = 2(min1 + min2)
            ribbonSum += length * width * height + wrap;
        }
        scanner.close();
        System.out.printf("You will need %d feet of wrapping paper.\nAnd %d feet of ribbon", presentSum, ribbonSum);
    }
}
