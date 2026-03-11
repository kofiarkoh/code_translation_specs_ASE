#include <stdio.h>
#include <stdlib.h>

int main()
{
    int n, k;
    long temp;
    long *data = (long *)malloc(n * sizeof(long));
    
    long cur = 1, last = 1;
    int grade = 0;
    scanf("%d %d", &n, &k);
    //initialization - 1st grade
    for (int i = 0; i < n; i++){
        scanf("%ld", &temp);
        data[i] = temp;
        if (i < k) {
            last *= temp;
        }
        else{
            cur = last / data[grade];
            cur *= temp;
            (cur > last)? printf("Yes\n") : printf("No\n");
            last = cur;
            grade++;
        }
    }
}