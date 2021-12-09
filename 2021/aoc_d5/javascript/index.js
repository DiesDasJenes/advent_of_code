var fs = require('fs');

const generateInputFromFile = (fileName) => {
	try {
		const data = fs.readFileSync(fileName, 'utf8');
		return data.toString().split('\n').map(singleLine => {
			let matches = singleLine.match(/([0-9]+),([0-9]+) -> ([0-9]+),([0-9]+)/)
			return {
				x1: parseInt(matches[1]),
				y1: parseInt(matches[2]),
				x2: parseInt(matches[3]),
				y2: parseInt(matches[4])
			}
		})
	} catch(e) {
		console.log('Error:', e.stack);
	}
}

const isDiagonal = (vector) => {
	return vector.x1 !== vector.x2 && vector.y1 !== vector.y2
}

const getPointsToCover = (number1, number2) => {
	const range = (min, max) => Array.from({ length: max - min + 1 }, (_, i) => min + i);

	let sortedArray = [number1, number2].sort((a, b) => {return a - b})
	return range(sortedArray[0], sortedArray[1])
}

const getPointsToCoverForDiagonal = (vector) => {
	// 1 - find out slope
	let m = (vector.y2 - vector.y1) / (vector.x2 - vector.x1)

	// 2 - find out "b" using slope and a point: y = mx + b
	let b = vector.y1 - (m * vector.x1)

	// generate coordinates using formula
	let allXCoordinatesToCover = getPointsToCover(vector.x1, vector.x2)

	return allXCoordinatesToCover.map(singleXCoordinate => {
		return `${singleXCoordinate}-${(m * singleXCoordinate) + b}`
	})
}

const getPointsCoveredByVector = (vector, isPartTwo) => {
	let pointsToCover = []

	if (isPartTwo && isDiagonal(vector)) {
		return getPointsToCoverForDiagonal(vector)
	} else {
		if(isDiagonal(vector)) {
			return null
		}
	}

	if (vector.x1 !== vector.x2) {
		pointsToCover = getPointsToCover(vector.x1, vector.x2).map(singleXValue => {
			return `${singleXValue}-${vector.y1}`
		})
	} else {
		pointsToCover = getPointsToCover(vector.y1, vector.y2).map(singleYValue => {
			return `${vector.x1}-${singleYValue}`
		})
	}

	return pointsToCover
}

const generateHashMap = (pointCoordinates, hashMap) => {
	pointCoordinates.forEach(singlePointCoordinate => {
		let currentValueOfPoint = hashMap.get(singlePointCoordinate)

		if (currentValueOfPoint) {
			currentValueOfPoint++
		} else {
			currentValueOfPoint = 1
		}

		hashMap.set(singlePointCoordinate, currentValueOfPoint)
	})


	return hashMap
}

const getAllCrossroadsFromHashMap = (hashMap) => {
	let countOfCrossroads = 0

	for (let singlePoint of hashMap) {
		if (singlePoint[1] >= 2) {
			countOfCrossroads++
		}
	}

	return countOfCrossroads
}

const solveChallenge = (filename, isPartTwo) => {
	const inputArray = generateInputFromFile(filename)

	let allPointsToIncrement = []
	for (let singleVector of inputArray) {
		let points = getPointsCoveredByVector(singleVector, isPartTwo)
		if (points !== null) {
			allPointsToIncrement = allPointsToIncrement.concat(points)
		}
	}
	
	let hashMap = new Map()
	hashMap = generateHashMap(allPointsToIncrement, hashMap)
	return getAllCrossroadsFromHashMap(hashMap)
}

const solvePartOne = (filename) => {
	return solveChallenge(filename, false)
}

const solvePartTwo = (filename) => {
	return solveChallenge(filename, true)
}

// RUN THE CHALLENGE

// pt1
// console.log('part 1: ', solvePartOne('input_final.txt'))

//pt 2
console.log('part 2: ', solvePartTwo('input_final.txt'))




module.exports = {
	generateInputFromFile,
	getPointsCoveredByVector,
	generateHashMap,
	getAllCrossroadsFromHashMap,
	solvePartOne,
	solvePartTwo,
	getPointsToCoverForDiagonal,
}