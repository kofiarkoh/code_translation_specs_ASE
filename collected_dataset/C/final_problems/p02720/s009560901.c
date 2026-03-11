#include <stdio.h>
#include <stdbool.h>

bool is_lunlun(unsigned long int x) {
    bool ans = true;
    if (x / 10 > 0) {
        unsigned long int temp = x % 10;
        x = x /10;
        while (x > 0) {
            if (x % 10 != temp && x % 10 + 1 != temp && temp + 1 != x % 10) {
                ans = false;
                break;
            }
            temp = x % 10;
            x = x / 10;
        }
    }
    return ans;
}

int lb(int x) {
    if (x == 0) return 0;
    else return x-1;
}

int ub(int x) {
    if (x == 0 || x == 9) return 9;
    else return x+1;
}

unsigned long int answer(int i[]) {
    unsigned long int ans = 0;
    int j;
    for (j=0; j<10; j++) {
        ans = ans * 10 + (unsigned long int)i[j];
    }
    return ans;
}

bool judge(int i[]) {
    int j;
    for (j=0; i[j] == 0; j++) ;
    bool ans = true;
    int temp = i[j];
    for (j++; j < 10; j++) {
        if (temp-i[j] > 1 || temp-i[j] < -1) {
            ans = false;
            break;
        }
        temp = i[j];
    }
    return ans;
}

int main(void) {
    int k;
    scanf("%d", &k);

    int count = -1;
    int i[10] = {0};
    unsigned long int ans;

    for (i[0] = 0; i[0] <= 9; i[0]++) {
        for (i[1] = lb(i[0]); i[1] <= ub(i[0]); i[1]++) {
            for (i[2] = lb(i[1]); i[2] <= ub(i[1]); i[2]++) {
                for (i[3] = lb(i[2]); i[3] <= ub(i[2]); i[3]++) {
                    for (i[4] = lb(i[3]); i[4] <= ub(i[3]); i[4]++) {
                        for (i[5] = lb(i[4]); i[5] <= ub(i[4]); i[5]++) {
                            for (i[6] = lb(i[5]); i[6] <= ub(i[5]); i[6]++) {
                                for (i[7] = lb(i[6]); i[7] <= ub(i[6]); i[7]++) {
                                    for (i[8] = lb(i[7]); i[8] <= ub(i[7]); i[8]++) {
                                        for (i[9] = lb(i[8]); i[9] <= ub(i[8]); i[9]++) {
                                            if (judge(i)) count += 1;
                                            if (count == k) {
                                                ans = answer(i);
                                                printf("%ld\n", ans);
                                                return 0;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    printf("Not founf\n");
    return 0;
}