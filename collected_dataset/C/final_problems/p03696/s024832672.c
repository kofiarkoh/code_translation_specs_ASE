#include<stdio.h>
#include<string.h>

int main(void) {
    int n;
    int i;
    scanf("%d", &n);
    char s[n];
    scanf("%s", s);
    int counterL = 0;
    int counterR = 0;
    int deleter = 0;
    for(i=0; i<n; i++) {
        if(s[i] == '(') {
            deleter++;
            continue;
        } else {
            if(deleter == 0) {
                counterL++;
            } else {
                deleter--;
            }
        }
    }
    deleter = 0;
    for(i=0; i<n; i++) {
        if(s[n-i-1] == ')') {
            deleter++;
        } else {
            if(deleter == 0) {
                counterR++;
            } else {
                deleter--;
            }
        }
    }
    char result[100];
    char left[100];
    char right[100];
    if(counterL > 0) {
        for(i=0; i<counterL; i++) {
            left[i] = '(';
        }
        strcat(left,s);
        strcpy(result, left);
    }

    if(counterR > 0) {
        for(i=0; i<counterR; i++) {
            right[i] = ')';
        }
        if(counterL > 0) {
            strcat(left, right);
            strcpy(result, left);
        } else {
            strcat(s, right);
            strcpy(result, s);
        }
    }

    if (counterL == 0 && counterR == 0) {
        printf("%s\n", s);
    } else {
        printf("%s\n", result);
    }
    return 0;
}
