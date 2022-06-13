import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class AoC_2015_6_1 {
    private static final boolean[][] field = new boolean[1000][1000];

    public static void main(String[] args) throws FileNotFoundException {
        File file = args.length != 0 ? new File(args[0]) : new File("file.txt");
        Scanner scanner = new Scanner(file);

        while (scanner.hasNext()) {
            String line = scanner.nextLine();
            Pattern pattern = Pattern.compile("(.*) (\\d+),(\\d+) through (\\d+),(\\d+)");
            Matcher matcher = pattern.matcher(line);

            if(matcher.find()){
                int x1 = Integer.parseInt(matcher.group(2));
                int y1 = Integer.parseInt(matcher.group(3));
                int x2 = Integer.parseInt(matcher.group(4));
                int y2 = Integer.parseInt(matcher.group(5));

                switch (matcher.group(1)) {
                    case "turn on":
                        turnOn(x1, y1, x2, y2);
                        break;
                    case "turn off":
                        turnOff(x1, y1, x2, y2);
                        break;
                    case "toggle":
                        toggle(x1, y1, x2, y2);
                        break;
                }
            }
        }
        scanner.close();
        System.out.printf("Number of lights on: %d", lightCount());
    }

    private static void toggle(int x1, int y1, int x2, int y2){
        for(int y = y1; y <= y2; y++) {
            for(int x = x1; x <= x2; x++) {
                field[y][x] = !field[y][x];
            }
        }
    }

    private static void turnOn(int x1, int y1, int x2, int y2){
        for(int y = y1; y <= y2; y++) {
            for(int x = x1; x <= x2; x++) {
                field[y][x] = true;
            }
        }
    }

    private static void turnOff(int x1, int y1, int x2, int y2){
        for(int y = y1; y <= y2; y++) {
            for(int x = x1; x <= x2; x++) {
                field[y][x] = false;
            }
        }
    }

    private static int lightCount(){
        int count = 0;
        for(int y = 0; y < 1000; y++) {
            for(int x = 0; x < 1000; x++) {
                if(field[y][x]){
                    count++;
                }
            }
        }
        return count;
    }
}
