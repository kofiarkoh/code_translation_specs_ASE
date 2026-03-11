#include <stdio.h>
#include <math.h>
int function(int);
int main(){
    int a,t;
    scanf("%d",&a);
    t = function(a / 100);
    if(t == 0){
        printf("ai1333");
    } else {
        printf("ai1333%d\n",t);
    }
    return 0;
}
int function(int num){
    double re,j;
    re = 0;
    for(j = num;j > 0;j--){
        re = re + 3 * pow(10, j-1);
    }
    return re;
}
