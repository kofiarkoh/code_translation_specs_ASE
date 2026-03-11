#include<stdio.h>

int ssort(int *arr,int N){
    static int buff[20000] = {};
    int high, low, max, i, p;
    high = 0;
    low = N - 1;
    max = 0;

    for(i=0; i<N; i++){
        if(arr[i] > max){
            max = arr[i];
            buff[high++] = arr[i];
        }else{
            buff[low--] = arr[i];
        }
    }
    p = 0;

    if(low == N-1){
        return 1;
    }else{
        for(i=N-1; i>low; i--){
            arr[p++] = buff[i];
        }
        for(i=0; i<high; i++){
            arr[p++] = buff[i];
        }
    }
    return 0;

}

int main(){
    int array[20000];
    int N,i;
    int count = 0;

    scanf("%d", &N);
    for(i=0; i<N; i++)
        scanf("%d", &array[i]);

    while(!ssort(array, N))
        count++;
    printf("%d", count);
    return 0;
}
