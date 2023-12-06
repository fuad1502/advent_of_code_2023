package aoc23_5;

import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Main {
	private static class Map {
		public long dstStart;
		public long srcStart;
		public long range;

		public Map(long dstStart, long srcStart, long range) {
			this.dstStart = dstStart;
			this.srcStart = srcStart;
			this.range = range;
		}

		public String toString() {
			return "(" + dstStart + "," + srcStart + "," + range + ")";
		}
	}

	public static void main(String[] args) throws IOException {
		{
			var seedsAndAllMaps = parse_input(new FileReader("aoc23_5/test"));
			var seeds = (List<Long>) seedsAndAllMaps[0];
			var allMaps = (List<List<Map>>) seedsAndAllMaps[1];
			assert f(seeds, allMaps) == 35;
		}
		var seedsAndAllMaps = parse_input(new FileReader("aoc23_5/input"));
		var seeds = (List<Long>) seedsAndAllMaps[0];
		var allMaps = (List<List<Map>>) seedsAndAllMaps[1];
		System.out.println("Result: " + f(seeds, allMaps));
	}

	private static long f(List<Long> seeds, List<List<Map>> allMaps) {
		long minimumSeedLocation = getSeedLocation(seeds.get(0), allMaps);
		for (var seed : seeds.stream().skip(1).toList()) {
			long seedLocation = getSeedLocation(seed, allMaps);
			minimumSeedLocation = Math.min(seedLocation, minimumSeedLocation);
		}
		return minimumSeedLocation;
	}

	private static long getSeedLocation(long seed, List<List<Map>> allMaps) {
		long currentSrc = seed;
		for (var xToYMaps : allMaps) {
			for (var map : xToYMaps) {
				if (currentSrc >= map.srcStart && currentSrc <= (map.srcStart + map.range)) {
					currentSrc = currentSrc - map.srcStart + map.dstStart;
					break;
				}
			}
		}
		return currentSrc;
	}

	private static Object[] parse_input(FileReader f)
			throws IOException {
		var br = new BufferedReader(f);
		// Extract seeds
		var seeds = Arrays.asList(br.readLine().split(":")[1].trim().split(" ")).stream()
				.map(e -> Long.parseLong(e)).toList();
		// Skip space
		br.readLine();
		// Fill maps
		var allMaps = new ArrayList<ArrayList<Map>>();
		for (long i = 0; i < 7; i++) {
			// Skip header
			br.readLine();
			// Read x-to-y maps
			var xToYMaps = new ArrayList<Map>();
			for (var line = br.readLine(); line != null && line.length() != 0; line = br.readLine()) {
				var entry = Arrays.asList(line.split(" ")).stream().map(e -> Long.parseLong(e))
						.toList();
				xToYMaps.add(new Map(entry.get(0), entry.get(1), entry.get(2)));
			}
			allMaps.add(xToYMaps);
		}
		var returnedObjects = new Object[2];
		returnedObjects[0] = seeds;
		returnedObjects[1] = allMaps;
		return returnedObjects;
	}
}
