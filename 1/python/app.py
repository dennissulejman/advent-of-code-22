with open("../input.txt") as input_file:
    highest_amount_of_calories_carried_by_elf = 0
    amount_of_calories_carried_by_current_elf = 0
    for current_line in input_file:

        # Remove unnecessary automatically added new lines
        current_line = current_line.rstrip("\n")
        if bool(current_line):
            amount_of_calories_carried_by_current_elf += int(current_line)
        else:
            highest_amount_of_calories_carried_by_elf =\
                amount_of_calories_carried_by_current_elf\
                if amount_of_calories_carried_by_current_elf > highest_amount_of_calories_carried_by_elf\
                else highest_amount_of_calories_carried_by_elf

            amount_of_calories_carried_by_current_elf = 0

    print(highest_amount_of_calories_carried_by_elf)
