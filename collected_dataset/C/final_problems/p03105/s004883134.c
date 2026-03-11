#include <stdio.h>

int main() {
    int a, b, c, cont = 0;
    int total;

    scanf("%d %d %d", &a, &b, &c);
    
    if (b / a  > c){
        total = c;
    }
    else {
        total = b / a;
    }
   
    

    
    printf("%d\n", total);

    return 0;
}