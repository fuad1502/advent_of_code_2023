package aoc23_11;

import java.io.BufferedReader;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.io.IOException;
import java.lang.reflect.Array;
import java.util.ArrayList;
import java.util.Arrays;

public class Main {
	public static void main(String[] args) throws FileNotFoundException, IOException {
		{
			var result = parseInput(new FileReader("aoc23_11/test"));
			var galaxyLocations = (ArrayList<Coordinate>) result[0];
			var voidRows = (ArrayList<Integer>) result[1];
			var voidCols = (ArrayList<Integer>) result[2];
			assert f(galaxyLocations, voidRows, voidCols, 2) == 374;
		}
		{
			var result = parseInput(new FileReader("aoc23_11/input"));
			var galaxyLocations = (ArrayList<Coordinate>) result[0];
			var voidRows = (ArrayList<Integer>) result[1];
			var voidCols = (ArrayList<Integer>) result[2];
			System.out.println("Result: " + f(galaxyLocations, voidRows, voidCols, 2));
		}
		{
			var result = parseInput(new FileReader("aoc23_11/test"));
			var galaxyLocations = (ArrayList<Coordinate>) result[0];
			var voidRows = (ArrayList<Integer>) result[1];
			var voidCols = (ArrayList<Integer>) result[2];
			assert f(galaxyLocations, voidRows, voidCols, 100) == 8410;
		}
		{
			var result = parseInput(new FileReader("aoc23_11/input"));
			var galaxyLocations = (ArrayList<Coordinate>) result[0];
			var voidRows = (ArrayList<Integer>) result[1];
			var voidCols = (ArrayList<Integer>) result[2];
			System.out.println("Result (part 2): " + f(galaxyLocations, voidRows, voidCols, 1000000));
		}
	}

	private static class Coordinate {
		long i;
		long j;

		Coordinate(long i, long j) {
			this.i = i;
			this.j = j;
		}

		public String toString() {
			return "(" + this.i + "," + this.j + ")";
		}
	}

	private static long f(ArrayList<Coordinate> galaxyLocations, ArrayList<Integer> voidRows,
			ArrayList<Integer> voidCols, int expansion) {
		// Expand galaxyLocations
		for (var loc : galaxyLocations) {
			var newI = loc.i;
			for (var row : voidRows) {
				if (row < loc.i) {
					newI += (expansion - 1);
				} else {
					break;
				}
			}
			loc.i = newI;
			var newJ = loc.j;
			for (var col : voidCols) {
				if (col < loc.j) {
					newJ += (expansion - 1);
				} else {
					break;
				}
			}
			loc.j = newJ;
		}
		// Calculate sum of distances
		long result = 0;
		for (int i = 0; i < galaxyLocations.size(); i += 1) {
			for (int j = i + 1; j < galaxyLocations.size(); j += 1) {
				result += distance(galaxyLocations.get(i), galaxyLocations.get(j));
			}
		}
		return result;
	}

	private static long distance(Coordinate a, Coordinate b) {
		return Math.abs(a.i - b.i) + Math.abs(a.j - b.j);
	}

	private static Object[] parseInput(FileReader file) throws IOException {
		var br = new BufferedReader(file);
		var galaxyLocations = new ArrayList<Coordinate>();
		var voidRows = new ArrayList<Integer>();
		Boolean[] isVoidCols = null;
		int i = 0;
		for (String line = br.readLine(); line != null; line = br.readLine()) {
			// Initialize isVoidCols
			if (isVoidCols == null) {
				isVoidCols = new Boolean[line.length()];
				Arrays.fill(isVoidCols, true);
			}
			// Add galaxy locations and find void rows and cols
			var isVoidRow = true;
			for (int j = 0; j < line.length(); j += 1) {
				if (line.charAt(j) == '#') {
					galaxyLocations.add(new Coordinate(i, j));
					isVoidRow = false;
					isVoidCols[j] = false;
				}
			}
			if (isVoidRow) {
				voidRows.add(i);
			}
			i += 1;
		}
		// Fill in voidCols from isVoidCols
		var voidCols = new ArrayList<Integer>();
		for (int c = 0; c < isVoidCols.length; c += 1) {
			if (isVoidCols[c]) {
				voidCols.add(c);
			}
		}

		var result = new Object[3];
		result[0] = galaxyLocations;
		result[1] = voidRows;
		result[2] = voidCols;
		return result;
	}
}
