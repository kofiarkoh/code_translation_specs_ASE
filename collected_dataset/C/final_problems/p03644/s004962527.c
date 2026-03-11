#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {

    int n;
    scanf("%d", &n);

    int arr[n];

    for (int i = n; i >= 1; i--) {
        
        int cnt = 0;

        int ntemp = i;

        while (ntemp >= 1) {
            
            if (ntemp % 2 == 0) {
                cnt++;
                ntemp /= 2;
            } else {
                break;
            }


        }
        arr[i] = cnt;

    }

    int maxdata = -1;
    int idx = 0;

    for (int i = 1; i <= n; i++) {

        //printf("%d %d\n", i, arr[i]);
        if (maxdata < arr[i]) {
            idx = i;
            maxdata = arr[i];
        }

    }

    printf("%d\n", idx);

    return 0;

}