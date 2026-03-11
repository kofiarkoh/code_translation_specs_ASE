#include <stdio.h>
#include <math.h>

int main(){
    int a,b;
    int i;
    int A,B;

    scanf("%d %d", &a, &b);
    A = a;
    B = b;

    for(i=1;i<b;i++){
        A += a * pow(10,i);
    }
    for(i=1;i<a;i++){
        B += b * pow(10, i);
    }
    if(a > b){
        printf("%d\n", B);
    }else{
        printf("%d\n", A);
    }

    return 0;

}