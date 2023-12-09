package aoc23_8;

import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.concurrent.TimeUnit;

public class Main {
	private static class Game {
		private String direction;
		private HashMap<String, String[]> map;
		private HashSet<String> startPositions;

		private Game(String direction, HashMap<String, String[]> map, HashSet<String> startPositions) {
			this.direction = direction;
			this.map = map;
			this.startPositions = startPositions;
		}
	}

	public static void main(String[] args) throws IOException {
		{
			var file = new FileReader("aoc23_8/test");
			var game = parse_input(file);
			assert f(game) == 2;
		}
		{
			var file = new FileReader("aoc23_8/test2");
			var game = parse_input(file);
			assert f(game) == 6;
		}
		{
			var file = new FileReader("aoc23_8/input");
			var game = parse_input(file);
			System.out.println("Result: " + f(game));
		}
		{
			var file = new FileReader("aoc23_8/test3");
			var game = parse_input(file);
			assert f2(game) == 6;
		}
		{
			var file = new FileReader("aoc23_8/input");
			var game = parse_input(file);
			System.out.println("Result (part 2): " + f2(game));
		}
	}

	private static int f(Game game) {
		var steps = 0;
		var directionIdx = 0;
		var position = "AAA";
		while (!position.equals("ZZZ")) {
			var entry = game.map.get(position);
			var direction = game.direction.charAt(directionIdx);
			if (direction == 'L') {
				position = entry[0];
			} else {
				position = entry[1];
			}
			directionIdx = (directionIdx + 1) % game.direction.length();
			steps += 1;
		}
		return steps;
	}

	private static long f2(Game game) {
		var steps = 0;
		var directionIdx = 0;
		var finish = false;
		var finishPositions = new ArrayList<Integer>();
		while (!finish) {
			var direction = game.direction.charAt(directionIdx);
			var newPositions = new HashSet<String>();
			finish = true;
			for (var position : game.startPositions) {
				var entry = game.map.get(position);
				if (direction == 'L') {
					position = entry[0];
				} else {
					position = entry[1];
				}
				if (position.charAt(2) != 'Z') {
					newPositions.add(position);
					finish = false;
				} else {
					finishPositions.add(steps + 1);
				}
			}
			game.startPositions = newPositions;
			directionIdx = (directionIdx + 1) % game.direction.length();
			steps += 1;
		}
		long totalLcm = lcm(finishPositions.get(0), finishPositions.get(1));
		for (int i = 2; i < finishPositions.size(); i++) {
			totalLcm = lcm(totalLcm, finishPositions.get(i));
		}
		return totalLcm;
	}

	private static long lcm(long a, long b) {
		return (a * b) / gcd(a, b);
	}

	private static long gcd(long a, long b) {
		while (b != 0) {
			var t = b;
			b = a % b;
			a = t;
		}
		return a;
	}

	private static Game parse_input(FileReader file) throws IOException {
		var reader = new BufferedReader(file);
		var map = new HashMap<String, String[]>();

		// Read direction
		var direction = reader.readLine();

		// Skip empty line
		reader.readLine();

		var startPositions = new HashSet<String>();
		for (String line; (line = reader.readLine()) != null;) {
			var from = line.substring(0, 3);
			var left = line.substring(7, 10);
			var right = line.substring(12, 15);
			String[] to = { left, right };
			map.put(from, to);
			if (from.charAt(2) == 'A') {
				startPositions.add(from);
			}
		}

		return new Game(direction, map, startPositions);
	}
}
