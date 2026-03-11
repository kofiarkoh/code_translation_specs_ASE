#include <stdio.h>


int main()
{
    int a, b, c, k;
    int g;
    scanf("%d", &a);
    scanf("%d", &b);
    scanf("%d", &c);
    scanf("%d", &k);

        if(k <= a + b){
            printf("%d\n", a);
        }
        else{
            g = a - (k-a-b);
            printf("%d\n", g);
        }
    
  

    return 0;
}