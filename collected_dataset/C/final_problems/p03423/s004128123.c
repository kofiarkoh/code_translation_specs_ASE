#include <stdio.h>
 
int main() {
    int n,tot=0;
    
    scanf("%d",&n);
    while(n>=3)
    {
        tot++;
        n=n-3;
    }
    printf("%d",tot);
}