const { it, expect } = require("@jest/globals")
const { generateInputFromFile,generateIndexTable,solvePartOne, solvePartTwo } = require("./index.js")


describe('Testing index functionaities', () => {
	it('part1: should split the input file into a valid object', () => {
		const inputObject = generateInputFromFile('input_example.txt')

		expect(inputObject.drawNumbers.length).toBe(27) // 27 numbers to be drawn
		expect(inputObject.tables.length).toBe(3) // 3 bingo tables to be read
		expect(inputObject.tables[0].length).toBe(5) // 5 rows for each table
		expect(inputObject.tables[0][0].length).toBe(5)// 5 columns for each row (5x5 matrix)
		expect(inputObject.tables[0][0][0].isMarked).toBe(false)// Flag for first position
		expect(inputObject.tables[0][0][0].value).toBe(22)// Value for first position
		expect(inputObject.tables[0][4][4].value).toBe(19)// Value for last position in first block
	})

	it('part1: should return a array with position of numbers', () => {
		const inputObject = generateInputFromFile('input_example.txt')
		const bagOfNumbers = generateIndexTable(inputObject.tables)

		expect(bagOfNumbers.length).toBe(27)
		expect(bagOfNumbers[7][0].x).toBe(2)
		expect(bagOfNumbers[7][0].y).toBe(4) 
		expect(bagOfNumbers[7][0].block).toBe(0)
	})

	// it('part1: should mark all numbers as true on their respective position', () => {
	// 	const inputObject = generateInputFromFile('input_example.txt')
	// 	const bagOfNumbers = generateIndexTable(inputObject.tables)
	
	// 	const updatedTables = markAllPositionOf(7,bagOfNumbers, inputObject.tables)
	
	// 	expect(updatedTables[0][2][4].isMarked).toBe(true)
	// 	expect(updatedTables[1][2][2].isMarked).toBe(true)
	// 	expect(updatedTables[2][4][4].isMarked).toBe(true)
	// })
	
	// it('part1: should find the bingo block in the first line', () => {
	// 	const inputObject = generateInputFromFile('input_example.txt')
	// 	const bagOfNumbers = generateIndexTable(inputObject.tables)
	
	// 	for (let i = 0; i <= 11; i++) {
	// 		inputObject.tables = markAllPositionOf(inputObject.drawNumbers[i], bagOfNumbers, inputObject.tables)
	// 	}
	
	// 	const blockIndex = findBingoBlock(inputObject.tables)
	
	// 	expect(blockIndex).toBe(2)
	// })
	//
	// it('part1: should find the bingo block if the bingo is in the middle column', () => {
	// 	const inputObject = generateInputFromFile('input_example.txt')
	// 	const bagOfNumbers = generateIndexTable(inputObject.tables)
	//
	// 	inputObject.drawNumbers = [0,15,13,7,10,9,16]
	//
	// 	for (let i = 0; i <= 6; i++) {
	// 		inputObject.tables = markAllPositionOf(inputObject.drawNumbers[i], bagOfNumbers, inputObject.tables)
	// 	}
	//
	// 	const blockIndex = findBingoBlock(inputObject.tables)
	//
	// 	expect(blockIndex).toBe(1)
	// })
	//
	// it('part1: should find the bingo block if the bingo is the first column', () => {
	// 	const inputObject = generateInputFromFile('input_example.txt')
	// 	const bagOfNumbers = generateIndexTable(inputObject.tables)
	//
	// 	inputObject.drawNumbers = [0,22,8,21,10,1,7,6]
	//
	// 	for (let i = 0; i <= 7; i++) {
	// 		inputObject.tables = markAllPositionOf(inputObject.drawNumbers[i], bagOfNumbers, inputObject.tables)
	// 	}
	//
	// 	const blockIndex = findBingoBlock(inputObject.tables)
	//
	// 	expect(blockIndex).toBe(0)
	// })


	it('part1: should solve part one for test example', () => {
		const resultPartOne = solvePartOne('input_example.txt')

		expect(resultPartOne).toBe(4512)
	})


	it('part2: should solve part two for test example', () => {
		const resultPartOne = solvePartTwo('input_example.txt')

		expect(resultPartOne).toBe(1924)
	})
})
