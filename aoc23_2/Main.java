package aoc23_2;

import java.io.BufferedReader;
import java.io.FileReader;
import java.util.ArrayList;

public class Main {
	public static void main(String[] args) {
		try {
			{
			    var games = Main.parseGame(new FileReader("aoc23_2/test"));
				var result = Main.f(games);
				var result2 = Main.f2(games);
				assert result == 8;
				assert result2 == 2286;
			}
			var games = Main.parseGame(new FileReader("aoc23_2/input"));
			var result = Main.f(games);
			var result2 = Main.f2(games);
			System.out.println("Result: " + result);
			System.out.println("Result (part 2): " + result2);
		} catch (Exception e) {
			e.printStackTrace();
		}
	}

	private static int f(ArrayList<ArrayList<int[]>> games) {
		var result = 0;
		var gameIdx = 1;
		for (var draws : games) {
			var invalid = false;
			for (var balls : draws) {
				if (balls[0] > 12 || balls[1] > 13 || balls[2] > 14) {
					invalid = true;
				}
			}
			if (!invalid) {
				result += gameIdx;
			}
			gameIdx += 1;
		}
		return result;
	}

	private static int f2(ArrayList<ArrayList<int[]>> games) {
		var result = 0;
		for (var draws : games) {
			var maxBalls = new int[3];
			for (var balls : draws) {
				for (int i = 0; i < 3; i++) {
					if (balls[i] > maxBalls[i]) {
						maxBalls[i] = balls[i];
					}
				}
			}
			result += maxBalls[0] * maxBalls[1] * maxBalls[2];
		}
		return result;
	}

	private static ArrayList<ArrayList<int[]>> parseGame(FileReader f) {
		// game -> draws
		// draw -> balls
		var games = new ArrayList<ArrayList<int[]>>();
		var br = new BufferedReader(f);
		String line;
		try {
			// Games loop
			while ((line = br.readLine()) != null) {
				var draws = new ArrayList<int[]>();
				var strDraws = line.split(":")[1].split(";");
				// Draws loop
				for (var draw : strDraws) {
					var balls = new int[3];
					var strBalls = draw.split(",");
					// Balls loop
					for (var ball : strBalls) {
						ball = ball.trim();
						var color = ball.split(" ")[1];
						var number = Integer.parseInt(ball.split(" ")[0]);
						switch (color) {
							case "red":
								balls[0] = number;
								break;
							case "green":
								balls[1] = number;
								break;
							default:
								balls[2] = number;
								break;
						}
					}
					draws.add(balls);
				}
				games.add(draws);
			}
			br.close();
		} catch (Exception e) {
			e.printStackTrace();
		}
		return games;
	}
}
