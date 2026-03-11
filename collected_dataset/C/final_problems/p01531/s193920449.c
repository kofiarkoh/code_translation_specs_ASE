#include <stdio.h>

int main(void)
{
    char inputString[1500] = {};
    char answer[100] = {};
    int i;
    int answerNumber = 0;
    
    scanf("%s", &inputString);
    for(i = 0; inputString[i] != '\0'; i++){
        if(i % 2 == 0){
            switch (inputString[i]){
            	case '1':
                    if(inputString[i + 1] == 'T'){
                        answer[answerNumber] = 'a';
                    }else if(inputString[i + 1] == 'L'){
                        answer[answerNumber] = 'i';
                    }else if(inputString[i + 1] == 'U'){
                        answer[answerNumber] = 'u';
                    }else if(inputString[i + 1] == 'R'){
                        answer[answerNumber] = 'e';
                    }else if(inputString[i + 1] == 'D'){
                        answer[answerNumber] = 'o';
                    }
                    i++;
                    break;
                case '2':
                    answer[answerNumber] = 'k';
                    break;
                case '3':
                    answer[answerNumber] = 's';
                    break;
                case '4':
                    answer[answerNumber] = 't';
                    break;
                case '5':
                    answer[answerNumber] = 'n';
                    break;
                case '6':
                    answer[answerNumber] = 'h';
                    break;
                case '7':
                    answer[answerNumber] = 'm';
                    break;
                case '8':
                    answer[answerNumber] = 'y';
                    break;
                case '9':
                    answer[answerNumber] = 'r';
                    break;
                case '0':
                    if(inputString[i + 1] == 'U'){
                        answer[answerNumber] = 'n';
                        answer[answerNumber + 1] = 'n';
                        answerNumber++;
                        i++;
                    }else{
                        answer[answerNumber] = 'w';
                    }
                    break;
                default:
                    break;
            }
            answerNumber++;
        }else{
            switch (inputString[i]){
                case 'T':
                    answer[answerNumber] = 'a';
                    break;
                case 'L':
                    answer[answerNumber] = 'i';
                    break;
                case 'U':
                    answer[answerNumber] = 'u';
                    break;
                case 'R':
                    answer[answerNumber] = 'e';
                    break;
                case 'D':
                    answer[answerNumber] = 'o';
                    break;
                default:
                    break;
            }
            answerNumber++;
        }
    } 
    answer[answerNumber] = '\n';
    printf("%s", answer); 
    
     
     
    return 0;
}