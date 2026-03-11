#include <stdio.h>
#include <string.h>
#include <math.h>

long long int ABS(long long int a)
{
    if(a>0)
        return a;
    else
        return -a;
}

int main()
{
    int n, i, j;
    int x[100], y[100], h[100];
    int max=0;
    int cx, cy, ch;

    scanf("%d",&n);

    for(i=0; i<n; i++) {
        scanf("%d %d %d",&x[i],&y[i],&h[i]);
        if(max<h[i]) {
            max = h[i];
        }
    }

    for(cx=0; cx<=100; cx++) {
        for(cy=0; cy<=100; cy++) {
            for(ch=max; ch<max+100; ch++) {
                int count=0;
                for(i=0; i<n; i++) {
                    if(ch == h[i] + ABS(cx-x[i]) + ABS(cy-y[i])) {
                        count++;
                    }
                }
                if(count==n){
                    printf("%d %d %d",cx, cy, ch);
                    return 0;
                }
            }
        }
    }

    return 0;
}