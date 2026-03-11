#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define LINE_BUF_SIZE 100

int main(){

    int i,j,k;
    int v;
    int c[3][3];
    int sum;

    sum = 0;

    for(i=0;i<3;i++){
        for(j=0;j<3;j++){
            //if(j != 3){
                scanf("%d", &v);
                c[i][j] = v;
                sum += v;
            //}
        }
    }

    if(sum % 3 == 0){
        printf("Yes");
    }

    else{
        printf("No");
    }

    return 0;

}