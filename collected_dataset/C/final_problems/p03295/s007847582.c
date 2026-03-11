#include <stdio.h>

#define LEN 100000

void sort(int numbers[LEN], int left, int right);

int main(void){
    int n, m;
    int a[LEN], b[LEN];
    int c = 0;
    int k, i;

    scanf("%d %d", &n, &m);
    for(k = 0; k < m; k++){
        scanf("%d %d", &a[k], &b[k]);
    }

    sort(a, 0, m-1);
    sort(b, 0, m-1);

    for(k = m-1; k >= 0; k--){
        for(i = 0; i < m; i++){
            if(b[k] < a[i]){
                c++;
            }
        }
    }

    if(c >= n){
        printf("%d\n", n-1);
    }else if(!c){
        printf("%d\n", 1);
    }else{
        printf("%d\n", c);
    }

    return 0;
}

void sort(int numbers[], int left, int right)
{
    int pivot, l_hold, r_hold;

    l_hold = left;
    r_hold = right;
    pivot = numbers[left];

    while (left < right){
        while ((numbers[right] >= pivot) && (left < right))
            right--;
        if (left != right){
            numbers[left] = numbers[right];
            left++;
        }
        while ((numbers[left] <= pivot) && (left < right))
            left++;
        if (left != right){
            numbers[right] = numbers[left];
            right--;
        }
    }

    numbers[left] = pivot;
    pivot = left;
    left = l_hold;
    right = r_hold;
    if (left < pivot){sort(numbers, left, pivot-1);}
    if (right > pivot){sort(numbers, pivot+1, right);}
}