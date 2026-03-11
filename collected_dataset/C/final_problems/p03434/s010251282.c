#include <stdio.h>
#include <string.h>
#include <math.h>

int main() {

    int n, a;

    scanf("%d", &n);

    int arr[n];

    for (int i = 0; i < n; i++) {
        scanf("%d", &a);
        arr[i] = a;
    }

    for (int i = 0; i < n - 1; i++) {
        for (int j = i + 1; j < n; j++) {
            //printf("%d %d\n", arr[i], arr[j]);
            if (arr[i] < arr[j]) {
                int temp = arr[i];
                arr[i] = arr[j];
                arr[j] = temp;
            }
        }
    }

    int sum01 = 0;
    int sum02 = 0;
    for (int i = 0; i < n; i++) {
        if (i % 2 == 0) {
            sum01 += arr[i];
        } else {
            sum02 += arr[i];
        }
    }

    printf("%d\n",sum01-sum02);


    return 0;
} 