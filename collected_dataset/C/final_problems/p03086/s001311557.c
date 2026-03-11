#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc,char* argv[]){
        char S[10];
        int i,max = 0,count=0;
        scanf("%s",S);
        for(i=0;i<strlen(S)-1;i++){
                switch(S[i]){
                        case 'A':
                        case 'C':
                        case 'T':
                        case 'G':
                        count++;
                        break;
                        default:
                        count = 0;
                        break;
                }
                
                if(max<count) max = count;
        }
        printf("%d\n",max);

}