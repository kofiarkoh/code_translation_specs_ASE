#include<stdio.h>
#include<stdlib.h>
#define LINE_BUF_SIZE 1024

int power(int a, int b) {
        int ret = 1;
        while (b--) {
                ret*=a;
        }
        return ret;
}

void parse(char line[], int a[], int n) {
        int pos_s = 0, pos_e = 0, i = 0, j;
        while (*line != '\0') {

                if (*line == ' ') {
                        // transfer
                        char tmp[pos_e - pos_s];
                        for (j=0; pos_s < pos_e; pos_s++, j++) {
                                tmp[j] = line[pos_s - pos_e];
                        }
                        a[i] = strtol(tmp, NULL, 10);
                        i++;
                        pos_s = pos_e+1;
                }
                pos_e++;
                line++;
        }
                        char tmp[pos_e - pos_s];
                        for (j=0; pos_s < pos_e; pos_s++, j++) {
                                tmp[j] = line[pos_s - pos_e];
                        }
                        a[i] = strtol(tmp, NULL, 10);
}
int func(int a[], int n) {
        int count = 0;
        while (1) {
                int count_ = count;
                for (int i=0; i<n; i++) {
                        if (a[i] % 2 == 0) {
                                a[i] = a[i]/2;
                                count += 1;
                                break;
                        }
                }
                if (count == count_) break;
        }
        return count;
}

int main() {
  char line[LINE_BUF_SIZE];
        int n;
  while (fgets(line, sizeof(line), stdin) != NULL) {
        if (sscanf(line, "%d", &n) == 1) {
                        int arr[n];
                        for (int i=0; i<n; i++) { arr[i] = 0; }
                while (fgets(line, sizeof(line), stdin) != NULL) {
                                parse(line, arr, n);
                                printf("%d", func(arr, n));
                        }
                        // printf("%d %d %d", arr[0], arr[1], arr[2]);
                }
        }
  return 0;
}