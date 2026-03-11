#include <stdio.h>
#include <stdlib.h>

#define TRUE 1
#define FALSE 0;

int xlist[100010];

int len[100010];

void qsort_v2(int d[], int s, int e)
{
    int i, j, p, temp;
    
    if(e - s <= 1) {
        return;
    }
    
    p = d[s];
    i = s;
    j = e - 1;
    
    while (TRUE) {
        
        while (d[i] < p && i < e)
            i++;
        
        while (p < d[j] && j > s)
            j--;
        if (i >= j)
            break;
        
        temp = d[i];
        d[i] = d[j];
        d[j] = temp;
        
        i++;
        j--;
        
    }
    
    qsort_v2(d, s, i);
    qsort_v2(d, j + 1, e);
    
    for(i = 0; i <= e; i++) {
        xlist[i] = d[i];
    }
}

int gcd(int x, int y)
{
    int r;
    
    while((r = x % y) != 0){
        x = y;
        y = r;
    }
    
    return y;
    
}



int main(void)
{
    int n, x;
    
    int i;
    
    int a;
    
    scanf("%d %d\n", &n, &x);
    
    for(i = 0; i < n; i++) {
        if(i == n - 1) {
            scanf("%d", &xlist[i]);
        } else {
            scanf("%d ", &xlist[i]);
        }
    }
    
    xlist[n] = x;
    
    qsort_v2(xlist, 0, n + 1);
    
    for(i = 0; i <= n; i++) {
        len[i] = xlist[i + 1] - xlist[i];
     }
    
    if(n == 1) {
        a = len[0];
    } else {
        for(i = 0; i < n - 1; i++) {
            if(i == 0) {
                a = gcd(len[i], len[i + 1]);
            } else {
                a = gcd(a, len[i + 1]);
            }
        }
    }
    
    printf("%d\n", a);

    return 0;
}
