import java.io.File;
import java.io.FileNotFoundException;
import java.math.BigInteger;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.Scanner;

public class AoC_2015_4_2 {
    public static void main(String[] args) throws NoSuchAlgorithmException, FileNotFoundException {
        Scanner scanner = args.length != 0 ? new Scanner(args[0]) : new Scanner(new File("file.txt"));
        String input = scanner.next();
        scanner.close();
        //omitted the first solution because AoC wouldn't accept it int count = 13981930;
        int count = 0;
        while(count < 1000000000) {
            String test = input+Integer.toHexString(count++);

            MessageDigest md = MessageDigest.getInstance("MD5");
            md.update(test.getBytes());
            String output = String.format("%032X", new BigInteger(1, md.digest()));

            if(output.substring(0,6).equals("000000")){
                System.out.printf("Hash found: %s\nTest case: %s\nSolution: %s",output, test, test.substring(input.length()));
                break;
            }
        }
    }
}
