const { it, expect } = require("@jest/globals")
const each = require("jest-each").default;
const { generateInputFromFile, getPointsCoveredByVector, generateHashMap, getAllCrossroadsFromHashMap, solvePartOne, solvePartTwo, getPointsToCoverForDiagonal } = require("./index.js")


describe('Testing index functionaities', () => {
	it('part1: should split the input file into a valid array', () => {
		const inputArray = generateInputFromFile('input_example.txt')
		const expectedFirstElement = { x1: 0, y1: 9, x2: 5, y2: 9 }
		console.log(inputArray)
		expect(inputArray.length).toBe(10)
		expect(inputArray[0]).toEqual(expectedFirstElement)
	})

	it('part1: should return a valid array of points to be incremented for a vector', () => {
		const vector = { x1: 0, y1: 9, x2: 5, y2: 9 }
		const result = getPointsCoveredByVector(vector, false)
		const expectedResult = ['0-9', '1-9', '2-9', '3-9', '4-9', '5-9']

		expect(result).toEqual(expectedResult)
	})

	it('part1: should add all point coordinates into a hash map', () => {
		const points = ['1-1','1-2','1-3','0-3','0-2','0-1']
		const hashMap = new Map()
		generateHashMap(points, hashMap)

		let expectedResult = new Map()
		expectedResult.set('1-1', 1);
		expectedResult.set('1-2', 1);
		expectedResult.set('1-3', 1);
		expectedResult.set('0-3', 1);
		expectedResult.set('0-2', 1);
		expectedResult.set('0-1', 1);

		expect(hashMap).toEqual(expectedResult)
	})

	it('part1: count all crossroads', () => {
		let hashMap = new Map()
		hashMap.set('1-1', 1);
		hashMap.set('1-2', 2);
		hashMap.set('1-3', 2);
		hashMap.set('0-3', 1);
		hashMap.set('0-2', 1);
		hashMap.set('0-1', 3);

		const result = getAllCrossroadsFromHashMap(hashMap)

		expect(result).toBe(3)
	})

	it('part1: should pass example', () => {
		const result = solvePartOne('input_example.txt')
		const expectedResult = 5

		expect(result).toBe(expectedResult)
	})

	it('part2: should return all points on a diagonal line', () => {
		const vector = { x1: 1, y1: 1, x2: 3, y2: 3 }
		const result = getPointsToCoverForDiagonal(vector)
		const expectedResult = ['1-1', '2-2', '3-3']

		expect(result).toEqual(expectedResult)
	})

	it('part2: should return all points on a diagonal line where x and y are inverted', () => {
		const vector = { x1: 9, y1: 7, x2: 7, y2: 9 }
		const result = getPointsToCoverForDiagonal(vector)

		expect(result.length).toBe(3)
		expect(result).toContain('9-7')
		expect(result).toContain('8-8')
		expect(result).toContain('7-9')
	})

	it('part2: should return all points on a diagonal line where x and y are inverted', () => {
		const vector = { x1: 3, y1: 6, x2: 6, y2: 3 }
		const result = getPointsToCoverForDiagonal(vector)

		expect(result.length).toBe(4)
		expect(result).toContain('3-6')
		expect(result).toContain('4-5')
		expect(result).toContain('5-4')
		expect(result).toContain('6-3')
	})

	it('part2: should return all points on a diagonal line when startpoint is higher', () => {
		const vector = { x1: 6, y1: 9, x2: 2, y2: 5 }
		const result = getPointsToCoverForDiagonal(vector)
		const expectedResult = ['2-5', '3-6', '4-7', '5-8', '6-9']

		expect(result).toEqual(expectedResult)
	})

	it('part2: should pass the example', () => {
		const result = solvePartTwo('input_example.txt')
		const expectedResult = 12

		expect(result).toEqual(expectedResult)
	})

})
