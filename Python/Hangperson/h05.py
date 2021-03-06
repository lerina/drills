#Step 5

import random
import art
import words

#DONE-1: - Update the word list to use the 'word_list' from hangman_words.py
#Delete this line: word_list = ["ardvark", "baboon", "camel"]
word_list = words.word_list

chosen_word = random.choice(word_list)
word_length = len(chosen_word)

end_of_game = False
lives = 6
stages = art.stages
clear = "\033c"

#DONE-3: - Import the logo from hangman_art.py and print it at the start of the game.
#Create blanks
display = []
for _ in range(word_length):
    display += "_"

msg = ""

while not end_of_game:
    print(clear)
    print(art.logo)
    #Testing code 
    print(f'Pssst, the solution is {chosen_word}.')
    print(msg)
    msg = ""

    guess = input("Guess a letter: ").lower()

    #DONE-4: - If the user has entered a letter they've already guessed, print the letter and let them know.
    if guess in display:
        msg = format(f"You've already guessed {guess}")

    #Check guessed letter
    for position in range(word_length):
        letter = chosen_word[position]
        #print(f"Current position: {position}\n Current letter: {letter}\n Guessed letter: {guess}")
        if letter == guess:
            display[position] = letter

    #Check if user is wrong.
    if guess not in chosen_word:
        #DONE-5: - If the letter is not in the chosen_word, print out the letter and let them know it's not in the word.
        msg = format(f"{guess} not in the word")
        lives -= 1
        if lives == 0:
            end_of_game = True
            msg += "\nYou lose."

    #Join all the elements in the list and turn it into a String.
    msg += format(f"\n{' '.join(display)}\n")

    #Check if user has got all letters.
    if "_" not in display:
        end_of_game = True
        msg += "\nYou win."

    #DONE-2: - Import the stages from hangman_art.py and make this error go away.
    msg += stages[lives]

print(clear)
print(art.logo)
print(msg + " ... bye")
