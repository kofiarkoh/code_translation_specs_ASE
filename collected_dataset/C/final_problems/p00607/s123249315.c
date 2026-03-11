#include <stdio.h>
#include <string.h>

int main(void)
{
    char message[256][1024];
    int i, len, nowx, nowy;
    int rowLength;
    char buffer[2], stack[1024], temp[1024];
    
    i = 0;
    memset(message, '\0', sizeof(message));
    memset(stack, '\0', sizeof(stack));
    while (1){
        scanf("%[^\n]", temp);
        getchar();
        if (strcmp(temp, "END_OF_TEXT") == 0){
            break;
        }
        strcpy(message[i], temp);
        
        message[i][strlen(message[i])] = '\n';
        i++;
    }
    len = i;
    nowx = nowy = 0;
    while (1){
        scanf("%s", buffer);
        
        if (buffer[0] == '-'){
            break;
        }
        
        switch (buffer[0]){
          case 'a':
            nowx = 0;
            break;
          
          case 'e':
            nowx = strlen(message[nowy]) - 1;
            break;
          
          case 'p':
            if (nowy > 0){
                nowy--;
            }
            nowx = 0;
            break;
          
          case 'n':
            if (nowy < len - 1){
                nowy++;
            }
            nowx = 0;
            break;
          
          case 'f':
            if (nowx < (int)strlen(message[nowy]) - 1){
                nowx++;
            }
            else if (nowy < len - 1 && nowx == (int)strlen(message[nowy]) - 1){
                nowy++;
                nowx = 0;
            }
            break;
          
          case 'b':
            if (nowx > 0){
                nowx--;
            }
            else if (nowy > 0 && nowx == 0){
                nowy--;
                nowx = strlen(message[nowy]) - 1;
            }
            break;
          
          case 'd':
            if (message[nowy][nowx] != '\n'){
                rowLength = strlen(message[nowy]) - 1;
                for (i = nowx; i < rowLength; i++){
                    message[nowy][i] = message[nowy][i + 1];
                }
                message[nowy][i] = '\0';
            }
            else if (message[nowy][nowx] == '\n' && nowy < len - 1){
                rowLength = strlen(message[nowy + 1]);
                for (i = 0; i < rowLength; i++){
                    message[nowy][nowx + i] = message[nowy + 1][i];
                }
                len -= 1;
                for (i = nowy + 1; i < len; i++){
                    strcpy(message[i], message[i + 1]);
                }
                memset(message[i], '\0', sizeof(message[i]));
            }
            break;
          
          case 'k':
            if (message[nowy][nowx] == '\n' && nowy < len - 1){
                rowLength = strlen(message[nowy + 1]);
                for (i = 0; i < rowLength; i++){
                    message[nowy][nowx + i] = message[nowy + 1][i];
                }
                len -= 1;
                for (i = nowy + 1; i < len; i++){
                    strcpy(message[i], message[i + 1]);
                }
                memset(message[i], '\0', sizeof(message[i]));
                memset(stack, '\0', sizeof(stack));
                stack[0] = '\n';
            }
            else if (message[nowy][nowx] != '\n'){
                memset(stack, '\0', sizeof(stack));
                strcpy(stack, &message[nowy][nowx]);
                memset(&message[nowy][nowx], '\0', sizeof(&message[nowy][nowx]));
                stack[strlen(stack) - 1] = '\0';
                rowLength = strlen(message[nowy]);
                nowx = rowLength;
                message[nowy][nowx] = '\n';
            }
            break;
          
          case 'y':
            if (stack[0] == '\n'){
                for (i = len; i > nowy; i--){
                    strcpy(message[i], message[i - 1]);
                }
                strcpy(message[nowy + 1], &message[nowy][nowx]);
                memset(&message[nowy][nowx], '\0', sizeof(&message[nowy][nowx]));
                message[nowy][nowx] = '\n';
                nowy++;
                nowx = 0;
                len++;
            }
            else if (stack[0] != '\0'){
                strcpy(temp, message[nowy]);
                strcpy(&message[nowy][nowx], stack);
                strcat(message[nowy], &temp[nowx]);
                nowx += strlen(stack);
            }
            break;
        }
    }
    
    for (i = 0; i < len; i++){
        printf("%s", message[i]);
    }
    
    return (0);
}