#include <stdio.h>

void print_all(int array[], int array_size) {
    int i;
    for(i=0; i<array_size; i++) {
        printf("%d ", array[i]);
    }
    printf("\n");
}

long int bubble_sort(int array[], int array_size) {
    int i, j, tmp;
    long int sum=0;
    for(i=0; i<array_size; i++) {
        // printf("ID: %d before sort %d\n", i, array[i]);
        for(j=array_size-1; j>i; j--) {
            if(array[j]>array[j-1]) {
                tmp = array[j];
                array[j] = array[j-1];
                array[j-1] = tmp;
            }
        }
        // printf("after sort %d\n", array[i]);
        if(i<2*array_size/3) if(i%2==1) sum += array[i];
    }
    return sum;
}

int main() {
    int N;
    scanf("%d", &N);

    int a[3*N], i;
    for(i=0; i<3*N; i++) {
        scanf("%d", &a[i]);
    }

    int tmp, j;
    long int sum=0;
    // printf("before sort\n");
    // print_all(a, 3*N);
    // printf("\n");
    sum = bubble_sort(a, 3*N);

    // printf("after sort\n");
    // print_all(a, 3*N);



    printf("%ld\n", sum);


    return 0;

}