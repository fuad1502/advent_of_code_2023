package aoc23_5;

import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
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
			return "(" + srcStart + "," + (srcStart + range - 1) + "," + (dstStart - srcStart) + ")";
		}
	}

	public static void main(String[] args) throws IOException {
		{
			var seedsAndAllMaps = parse_input(new FileReader("aoc23_5/test"), false);
			var seedLine = (String) seedsAndAllMaps[0];
			var allMaps = (List<List<Map>>) seedsAndAllMaps[1];
			assert f(seedLine, allMaps) == 35;
		}
		{
			var seedsAndAllMaps = parse_input(new FileReader("aoc23_5/test"), false);
			var seedLine = (String) seedsAndAllMaps[0];
			var allMaps = (List<List<Map>>) seedsAndAllMaps[1];
			assert f2(seedLine, allMaps) == 46;
		}
		{
			var seedsAndAllMaps = parse_input(new FileReader("aoc23_5/input"), false);
			var seedLine = (String) seedsAndAllMaps[0];
			var allMaps = (List<List<Map>>) seedsAndAllMaps[1];
			System.out.println("Result: " + f(seedLine, allMaps));
		}
		{
			var seedsAndAllMaps = parse_input(new FileReader("aoc23_5/input"), false);
			var seedLine = (String) seedsAndAllMaps[0];
			var allMaps = (List<List<Map>>) seedsAndAllMaps[1];
			System.out.println("Result: " + f2(seedLine, allMaps));
		}
	}

	private static long f(String seedLine, List<List<Map>> allMaps) {
		var seeds = parseSeedLine(seedLine);
		long minimumSeedLocation = getSeedLocation(seeds.get(0), allMaps);
		for (var seed : seeds) {
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

	private static long f2(String seedLine, List<List<Map>> allMaps) {
		var xRanges = parseSeedLine2(seedLine);
		var sortedAllMaps = sort(allMaps);
		for (var xToYMaps : sortedAllMaps) {
			xRanges = transformAllRangesFromXToY(xRanges, xToYMaps);
		}
		return minimumInRanges(xRanges);
	}

	private static List<List<Long>> transformAllRangesFromXToY(List<List<Long>> xRanges, List<Map> xToYMaps) {
		var allTransformedRanges = new ArrayList<List<Long>>();
		for (var xRange : xRanges) {
			var transformedRanges = transformSingleRangeFromXToY(xRange, xToYMaps);
			allTransformedRanges.addAll(transformedRanges);
		}
		return allTransformedRanges;
	}

	private static List<List<Long>> transformSingleRangeFromXToY(List<Long> xRange, List<Map> xToYMaps) {
		var transformedRanges = new ArrayList<List<Long>>();
		var lowerMapIdx = mapIdx(xRange.get(0), xToYMaps);
		var upperMapIdx = mapIdx(xRange.get(1), xToYMaps);
		if (lowerMapIdx == upperMapIdx) {
			// Add only range
			var mapIdx = lowerMapIdx;
			var range = new ArrayList<Long>();
			if (mapIdx % 2 == 0) {
				// direct
				range.add(xRange.get(0));
				range.add(xRange.get(1));
			} else {
				// map
				var map = xToYMaps.get(mapIdx / 2);
				range.add(xRange.get(0) - map.srcStart + map.dstStart);
				range.add(xRange.get(1) - map.srcStart + map.dstStart);
			}
			if (range.get(0) < range.get(1)) {
				transformedRanges.add(range);
			}
			lowerMapIdx += 1;
			upperMapIdx -= 1;
		} else {
			// Add first range
			var firstRange = new ArrayList<Long>();
			if (lowerMapIdx % 2 == 0) {
				// direct
				var lowerMap = xToYMaps.get((lowerMapIdx + 1) / 2);
				firstRange.add(xRange.get(0));
				firstRange.add(lowerMap.srcStart - 1);
			} else {
				// map
				var lowerMap = xToYMaps.get(lowerMapIdx / 2);
				firstRange.add(xRange.get(0) - lowerMap.srcStart + lowerMap.dstStart);
				firstRange.add(lowerMap.dstStart + lowerMap.range - 1);
			}
			if (firstRange.get(0) < firstRange.get(1)) {
				transformedRanges.add(firstRange);
			}
			lowerMapIdx += 1;
			// Add last range
			var lastRange = new ArrayList<Long>();
			if (upperMapIdx % 2 == 0) {
				// direct
				var upperMap = xToYMaps.get((upperMapIdx - 1) / 2);
				lastRange.add(upperMap.srcStart + upperMap.range);
				lastRange.add(xRange.get(1));
			} else {
				// map
				var upperMap = xToYMaps.get(upperMapIdx / 2);
				lastRange.add(upperMap.dstStart);
				lastRange.add(xRange.get(1) - upperMap.srcStart + upperMap.dstStart);
			}
			if (lastRange.get(0) < lastRange.get(1)) {
				transformedRanges.add(lastRange);
			}
			upperMapIdx -= 1;
		}
		// Add center ranges
		for (int i = lowerMapIdx; i <= upperMapIdx; i += 1) {
			var range = new ArrayList<Long>();
			if (i % 2 == 0) {
				// direct
				var previousMap = xToYMaps.get((i - 1) / 2);
				var nextMap = xToYMaps.get((i + 1) / 2);
				range.add(previousMap.srcStart + previousMap.range);
				range.add(nextMap.srcStart - 1);
			} else {
				// map
				var currentMap = xToYMaps.get(i / 2);
				range.add(currentMap.dstStart);
				range.add(currentMap.dstStart + currentMap.range - 1);
			}
			if (range.get(0) < range.get(1)) {
				transformedRanges.add(range);
			}
		}
		return transformedRanges;
	}

	private static List<List<Map>> sort(List<List<Map>> allMaps) {
		Comparator<Map> comparator = (Map m1, Map m2) -> {
			return Long.compare(m1.srcStart, m2.srcStart);
		};
		for (var xToYMaps : allMaps) {
			xToYMaps.sort(comparator);
		}
		return allMaps;
	}

	private static long minimumInRanges(List<List<Long>> ranges) {
		long min = ranges.get(0).get(0);
		for (var range : ranges) {
			min = Math.min(range.get(0), min);
		}
		return min;
	}

	private static int mapIdx(long x, List<Map> xToYMaps) {
		int idx = 0;
		for (var map : xToYMaps) {
			if (x >= map.srcStart) {
				idx += 1;
			} else {
				break;
			}
			if (x >= map.srcStart + map.range) {
				idx += 1;
			} else {
				break;
			}
		}
		return idx;
	}

	private static Object[] parse_input(FileReader f, Boolean isSecondPart)
			throws IOException {
		var br = new BufferedReader(f);
		// Extract seed line
		var seedLine = br.readLine().split(":")[1];
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
		returnedObjects[0] = seedLine;
		returnedObjects[1] = allMaps;
		return returnedObjects;
	}

	private static List<Long> parseSeedLine(String seedLine) {
		return Arrays.asList(seedLine.trim().split(" ")).stream().map(e -> Long.parseLong(e)).toList();
	}

	private static List<List<Long>> parseSeedLine2(String seedLine) {
		var numbers = Arrays.asList(seedLine.trim().split(" ")).stream().map(e -> Long.parseLong(e)).toList();
		var ranges = new ArrayList<List<Long>>();
		for (int i = 0; i < numbers.size(); i += 2) {
			var range = new ArrayList<Long>();
			range.add(numbers.get(i));
			range.add(numbers.get(i) + numbers.get(i + 1) - 1);
			ranges.add(range);
		}
		return ranges;
	}
}
