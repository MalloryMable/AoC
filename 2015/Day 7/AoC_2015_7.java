import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.Scanner;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class AoC_2015_7 {

    //returns value for a memory position or initializes to zero
    private static int function(String func, int a, int b){
        switch (func){
            case "AND":
                return a & b;
            case "OR":
                return a ^ b;
            case "LSHIFT":
                return a << b;
            case "RSHIFT":
                return a >> b;
            case "NOT":
                return ~a;
            case "SET":
                return a;
            default:
                throw new IllegalArgumentException("Invalid Function");
        }
    }

    private static int get(String key){
        if(isNumeric(key)){
            return Integer.parseInt(key);
        }
        if(keys.contains(key)){
            return values.get(keys.indexOf(key));
        }
        keys.add(key);
        values.add(0);
        return 0;
    }

    private static void set(String key, int value){
        if(keys.contains(key)){
            values.set(keys.indexOf(key), value);
            return;
        }
        keys.add(key);
        values.add(value);
    }

    private static boolean isNumeric(String string) {
        try {
            Integer.parseInt(string);
            return true;
        } catch (NumberFormatException ignored) {
            return false;
        }
    }

    private static final ArrayList<String> keys = new ArrayList<>();
    private static final ArrayList<Integer> values = new ArrayList<>();

    public static void main(String[] args) throws FileNotFoundException {
        File file = args.length != 0 ? new File(args[0]) : new File("file.txt");
        Scanner scanner = new Scanner(file);
        ArrayList<String> functions = new ArrayList<>();
        ArrayList<String[]> inputs = new ArrayList<>();
        ArrayList<String> dest = new ArrayList<>();

        while (scanner.hasNext()) {
            String line = scanner.nextLine();
            Pattern twoInputs = Pattern.compile("(.+) (\\w+) (.+) -> (.+)");
            Matcher twoMatch = twoInputs.matcher(line);
            Pattern oneInput = Pattern.compile("\\w+ (.+) -> (.+)");
            Matcher oneMatch = oneInput.matcher(line);
            Pattern setValue = Pattern.compile("(.+) -> (.+)");
            Matcher valueMatch = setValue.matcher(line);

            //find seems to initialize the matcher
            if(twoMatch.find()){
                String inputOne = twoMatch.group(1);
                String inputTwo = twoMatch.group(3);
                String function = twoMatch.group(2);
                functions.add(function);
                inputs.add(new String[]{inputOne, inputTwo});
                dest.add(twoMatch.group(4));
            }else if(oneMatch.find()){
                //WILL THIS ALWAYS BE NOT?
                String input = oneMatch.group(2);
                functions.add("NOT");
                inputs.add(new String[]{input, ""});
                dest.add(oneMatch.group(3));
            }else if(valueMatch.find()){
                functions.add("SET");
                inputs.add(new String[]{valueMatch.group(1), ""});
                dest.add(valueMatch.group(2));
            }
        }
        scanner.close();
        //propagates signal through the circuit
        for(int j = 0; j < 25; j++){
            for (int i = 0; i < functions.size(); i++) {
                set(dest.get(i), function(functions.get(i), get(inputs.get(i)[0]), get(inputs.get(i)[1])));
            }
            System.out.println("New line");
            for(String key: keys) {
                System.out.printf("  %s: %03d\n",key, get(key));
            }
        }

        System.out.printf("The value stored to wire a is %d\n", get("a"));

        //Change value feeding into b
        int index = dest.indexOf("b");
        functions.set(index, "SET");
        inputs.set(index, new String[]{String.valueOf(get("a")), ""});

        //propagates signal through the circuit
        for(int j = 0; j < 25; j++){
            for (int i = 0; i < functions.size(); i++) {
                set(dest.get(i), function(functions.get(i), get(inputs.get(i)[0]), get(inputs.get(i)[1])));

            }

        }

        System.out.printf("The value stored to wire a after we override wire b %d\n", get("a"));
    }
}
