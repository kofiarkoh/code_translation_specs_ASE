//
//  main.c
//  AtCoderProblems
//
//  Created by Gaska Wojtek on 2020/05/21.
//  Copyright © 2020 Gaska Wojtek. All rights reserved.
//

#include <stdio.h>
#include <string.h>
#include <stdlib.h>

void ReadInput(char * inputString)
{
    //printf("Enter your ID: \n");
    scanf("%s", inputString);
    
    while (strlen(inputString) > 10)
    {
        //printf("Wrong input \n");
        ReadInput(inputString);
        if(strlen(inputString) < 10)
        {
          break;
        }
    }
}

void CheckLength(char* firstString, char* secondString)
{
    for(int i = 0; i < strlen(firstString); i++)
    {
        if(firstString[i] != secondString[i])
        {
            printf("No \n");
            return;
        }
    }
    
    if(strlen(secondString) == (strlen(firstString) + 1))
    {
        printf("Yes \n");
    }
    else
    {
        printf("No \n");
    }
        
}

int main(int argc, const char * argv[])
{

    char *firstString = malloc(10 * sizeof(char));
    char *secondString = malloc(10 * sizeof(char));
    
    ReadInput(firstString);
    //printf("ID is already taken...\n");
    ReadInput(secondString);

    CheckLength(firstString, secondString);
    
    return 0;
}
