#include <stdio.h>
#include <string.h>
#include <stdlib.h>

typedef int keytype;
//quicksort(<#keytype *a#>, <#int first#>, <#int last#>)

void quicksort(keytype a[], int first, int last){
    int i, j;
    keytype x, t;
    
    x = a[(first + last) / 2];
    i = first;  j = last;
    for ( ; ; ) {
        while (a[i] < x) i++;
        while (x < a[j]) j--;
        if (i >= j) break;
        t = a[i];  a[i] = a[j];  a[j] = t;
        i++;  j--;
    }
    if (first  < i - 1) quicksort(a, first , i - 1);
    if (j + 1 < last) quicksort(a, j + 1, last);
}

int main(void){
    int n,data[100000],a[100000],i,j,tmp,k=0;
    scanf("%d",&n);
    for (i=0; i<n; i++) {
        scanf("%d",&data[i]);
    }
    
    for (i = 1; i < n; i++) {
        tmp = data[i];
        a[i]=i;
        k=a[i];
        if (data[i - 1] > tmp) {
            j = i;
            do {
                data[j] = data[j - 1];
                a[j]=a[j-1];
                j--;
            } while (j > 0 && data[j - 1] > tmp);
            data[j] = tmp;
            a[j]=k;
        }
    }
    
    for (i=0; i<n; i++) {
        printf("%d ",a[i]+1);
    }
    
    return 0;
}

