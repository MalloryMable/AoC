import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Scanner;

public class AoC_2015_3_2 {
    public static void main(String[] args) throws FileNotFoundException {

        File file = args.length != 0 ? new File(args[0]) : new File("file.txt");
        Scanner scanner = new Scanner(file);
        //NOTE: this program only accepts one line of input of arbitrary length
        String line = scanner.nextLine();

        ArrayList<ArrayList<Integer>> field = new ArrayList<>(0);
        int y, x = 0, x1 = 0, y1 = 0, x2 = 0, y2 = 0, visited = 1;
        //initializing
        field.add(new ArrayList<>());
        ArrayList<Integer> row = field.get(x);
        row.add(1);


        for(int letter = 0; letter < line.length(); letter++) {
            int presentCount;
            if(letter % 2 == 0){
                switch (line.charAt(letter)) {
                    case '>':
                        x1++;
                        break;
                    case '<':
                        x1--;
                        break;
                    case '^':
                        y1++;
                        break;
                    case 'v':
                        y1--;
                        break;
                }
                x = x1;
                y = y1;
            } else {
                switch (line.charAt(letter)) {
                    case '>':
                        x2++;
                        break;
                    case '<':
                        x2--;
                        break;
                    case '^':
                        y2++;
                        break;
                    case 'v':
                        y2--;
                        break;
                }
                x = x2;
                y = y2;
            }

            //If the field would be negative offset the entire grid
            if(y < 0) {
                field.add(0, new ArrayList<>(Collections.nCopies(row.size(), 0)));
                y++;
                y1++;
                y2++;

            }
            //Further ensure the row exists
            if(y >= field.size()) {
                field.add(new ArrayList<>(Collections.nCopies(x, 0)));
            }

            //Move to current row
            row = field.get(y);


            //Ensures the collum exists for current row
            if(x < 0) {
                for(ArrayList<Integer> localRow: field){
                    localRow.add(0, 0);
                }
                x++;
                x1++;
                x2++;

            }
            while(x >= row.size()) {
                row.add(0);
            }

            //If this is the first present add to the number of houses visted
            presentCount = row.get(x);
            if(presentCount == 0) {
                visited++;
            }

            row.set(x, ++presentCount);
        }

        System.out.printf("Together the Santas visits %d houses", visited);
    }
}
