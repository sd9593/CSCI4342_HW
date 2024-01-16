#Description: This program takes in a binary file (which lists fixed-length values each on a separate line) and passes it to LoadDiagnostics,
# which reads it and passes it to CheckPower. CheckPower takes in the list of binary numbers and computes gamma rate (most common digit in 
# that that numerical place), epsilon rate (least common digit in that numerical place), and overall power rate (gamma rate multipled by 
# epsilon rate). The power rate is returned and printed to terminal.

import sys

inputFile = ""
fileContent = []

def Main():
#calls all other functions and prints returned values
	inputFile = sys.argv[1] #takes file argument from command line
	LoadDiagnostics(inputFile)
	print ("Loading diagnostics...")
	CheckPower(fileContent)

def LoadDiagnostics(inputFile):
#takes fileName input and retrieves, reads file to return list of binary values in file
	with open(inputFile, 'r') as file:
		for line in file:
			fileContent.append(line.strip())
	file.close()
	return fileContent

def CheckPower(fileContent):
#takes list of binary values, calculates gamma rate, epsilon rate, returns power rate
	gamma = ""
	count = 0
	element = 0
	epsilon = ""
	powerRate = 0
	#gamma rate is number comprised of most common bit in each place (assume 1 if equal)
	while element < len(fileContent[0]):
		for item in fileContent:
			if item[element:element+1] == '1':
				count = count+1
		if count >= len(fileContent)/2:
			gamma = gamma + "1"
		else:
			gamma = gamma + "0"
		element = element + 1
		count = 0
	print("Gamma rate computed...")
	element = 0
	#epsilon rate is number comprised of least common bit in each place (assume 1 if equal)
	while element < len(fileContent[0]):
		for item in fileContent:
			if item[element:element+1] == '1':
				count = count+1
		if count <= len(fileContent)/2:
			epsilon = epsilon + "1"
		else:
			epsilon = epsilon + "0"
		element = element + 1
		count = 0
	print("Epsilon rate computed...")
	#power rate is epsilon rate multiplied by gamma rate
	powerRate = int(gamma,2)*int(epsilon,2) #converts bin to dec
	print("Power consumption rate: " + str(powerRate))

Main()
