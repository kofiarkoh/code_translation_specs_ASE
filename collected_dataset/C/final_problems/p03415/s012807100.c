#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <ctype.h>
#include <string.h>

int main(){

        char original[3][3],nova[1][3];

        for(int i=0;i<3;i++){
            for(int j=0;j<3;j++){
                scanf(" %c",&original[i][j]);
                if(i==j){
                    printf("%c",original[i][j]);
                }
            }
        }
        puts("");


    return 0;
}
