package aoc23_14;

import java.io.BufferedReader;
import java.io.FileReader;
import java.util.ArrayList;

class Main {
	public static void main(String[] args) throws Exception {
		{
			var t = parseInput(new FileReader("aoc23_14/test"));
			var allBlockedBoulders = (ArrayList<ArrayList<BlockedBoulders>>) t[0];
			var numberOfRows = (int) t[1];
			assert f(allBlockedBoulders, numberOfRows) == 136;
		}
		{
			var t = parseInput(new FileReader("aoc23_14/input"));
			var allBlockedBoulders = (ArrayList<ArrayList<BlockedBoulders>>) t[0];
			var numberOfRows = (int) t[1];
			System.out.println(f(allBlockedBoulders, numberOfRows));
		}
	}

	private static class BlockedBoulders {
		private int location;
		private int numberOfBoulders;

		BlockedBoulders(int location, int numberOfBoulders) {
			this.location = location;
			this.numberOfBoulders = numberOfBoulders;
		}
	}

	private static int f(ArrayList<ArrayList<BlockedBoulders>> allBlockedBoulders, int numberOfRows) {
		int result = 0;
		for (var blockedBoulders : allBlockedBoulders) {
			result += calculatePoint(blockedBoulders, numberOfRows);
		}
		return result;
	}

	private static int calculatePoint(ArrayList<BlockedBoulders> blockedBoulders, int numberOfRows) {
		int result = 0;
		for (var blockedBoulder : blockedBoulders) {
			result += calculatePoint(blockedBoulder, numberOfRows);
		}
		return result;
	}

	private static int calculatePoint(BlockedBoulders blockedBoulder, int numberOfRows) {
		var a = numberOfRows - (blockedBoulder.location);
		var b = numberOfRows - (blockedBoulder.location + blockedBoulder.numberOfBoulders - 1);
		var n = blockedBoulder.numberOfBoulders;
		return ((a + b) * n) / 2;
	}

	private static Object[] parseInput(FileReader file) throws Exception {
		BufferedReader reader = new BufferedReader(file);
		var allBlockedBoulders = new ArrayList<ArrayList<BlockedBoulders>>();
		int row = 0;
		for (String line = reader.readLine(); line != null; line = reader.readLine()) {
			if (allBlockedBoulders.size() == 0) {
				for (int i = 0; i < line.length(); i++) {
					var blockedBoulders = new ArrayList<BlockedBoulders>();
					blockedBoulders.add(new BlockedBoulders(0, 0));
					allBlockedBoulders.add(blockedBoulders);
				}
			}
			for (int i = 0; i < line.length(); i++) {
				if (line.charAt(i) == '#') {
					allBlockedBoulders.get(i).add(new BlockedBoulders(row + 1, 0));
				} else if (line.charAt(i) == 'O') {
					allBlockedBoulders.get(i)
							.get(allBlockedBoulders.get(i).size() - 1).numberOfBoulders++;
				}
			}
			row++;
		}
		var numberOfRows = row;
		var returnArray = new Object[2];
		returnArray[0] = allBlockedBoulders;
		returnArray[1] = numberOfRows;
		return returnArray;
	}
}
