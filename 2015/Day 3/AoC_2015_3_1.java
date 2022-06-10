import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Scanner;

public class AoC_2015_3_1 {

    public static void main(String[] args) throws FileNotFoundException {

        File file = args.length != 0 ? new File(args[0]) : new File("file.txt");
        Scanner scanner = new Scanner(file);
        //NOTE: this program only accepts one line of input of arbitrary length
        String line = scanner.nextLine();

        ArrayList<ArrayList<Integer>> field = new ArrayList<>();
        int x = 0, y = 0, visited = 1;
        //initializing
        field.add(new ArrayList<>());
        ArrayList<Integer> row = field.get(x);
        row.add(1);


        for(int letter = 0; letter < line.length(); letter++) {
            int presentCount;
            switch(line.charAt(letter)){
                case '>':
                    x++;
                    break;
                case '<':
                    x--;
                    break;
                case '^':
                    y++;
                    break;
                case 'v':
                    y--;
                    break;
            }
            if(y < 0) {
                field.add(0, new ArrayList<>(Collections.nCopies(x, 0)));
                y++;
            }
            //First ensure the row exists
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
            }
            while(x >= row.size()) {
                row.add(0);
            }

            presentCount = row.get(x);
            if(presentCount == 0) {
                visited++;
            }

            row.set(x, ++presentCount);

        }

        System.out.printf("Santa visits %d houses", visited);
    }
}
