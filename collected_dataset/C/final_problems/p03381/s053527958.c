
#include<stdio.h>

void sort(int data[], int n){
    int temp;
    for(int i = 0; i < n; i++){
        for(int j = i + 1; j < n; j++){
            if(data[i] < data[j]){
                temp = data[i];
                data[i] = data[j];
                data[j] = temp;
            }
        }
    }
}

int main(void){
    int data1[200000], data2[200000], n;
    scanf("%d", &n);
    for(int i = 0; i < n; i++){
        scanf("%d", &data1[i]);
    }
    for(int i = 0; i < n; i++){
        for(int j = 0; j < n; j++){
            data2[j] = data1[j];
        }
        data2[i] = 0;
        sort(data2, n);
        printf("%d\n", data2[n / 2 - 1]);
    }
    return 0;
}