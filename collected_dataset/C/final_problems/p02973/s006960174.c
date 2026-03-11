#include <stdio.h>
#define N 100000

typedef int key;

void marge(key data[], key work[], int start, int mid, int end){
    int i = start;
    int j = mid + 1;
    int k = start;
    while(i <= mid && j <= end){
        if(data[i] >= data[j]){
            work[k++] = data[i++];
        }else{
            work[k++] = data[j++];
        }
    }
    
    if(i <= mid){
        while(i <= mid){
            work[k++] = data[i++];
        }
    }else{
        while(j <= end){
            work[k++] = data[j++];
        }
    }
    
    for(k = start; k <= end; k++){
        data[k] = work[k];
    }
}

void margesort(key data[], key work[], int start, int end){
    int mid;
    if(start < end){
        mid = start + (end - start)/2;
        margesort(data, work, start, mid);
        margesort(data, work, mid + 1, end);

        marge(data, work, start, mid, end);
    }
}

int main(void){
    int n;
    int a[N];
    int memo[N];
    int work[N];
    int i,j;
    int count = 0;
    int maxkey = 0;
    for(i = 0; i < N; i++){
        memo[i] = -1;
    }
    
    scanf("%d", &n);
    for(i = 0; i < n; i++){
        scanf("%d", &a[i]);
    }
    
    memo[0] = a[0];
    for(i = 1; i < n; i++){
        j = 0;
        while(a[i] <= memo[j]){
            j++;
        }
        memo[j] = a[i];
        margesort(memo, work, 0, n-1);
    }
    
    for(i = 0; ; i++){
        if(memo[i] == -1){
            break;
        }
    }
    
    printf("%d", i);
    return 0;
}