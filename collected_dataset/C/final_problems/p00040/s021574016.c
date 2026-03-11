#include <stdio.h>
#include <string.h>
#include <ctype.h>

int gcd(int a, int b)
{
    if (b == 0) return a;
    return gcd(b, a % b);
}

void affine_decode(char* s, int* dec)
{
    int i;

    for (i = 0; s[i]; ++i){
        if (!isalpha(s[i])) continue;
        s[i] = dec[s[i] - 'a'] + 'a';
    }
}

int is_correct(char* s)
{
    int i;

    for (i = 0; s[i]; ++i){
        if (!strncmp(&s[i], "this", 4) || !strncmp(s + i, "that", 4)){
            return 1;
        }
    }
    return 0;
}

int main(void)
{
    int n;
    int i, j, k;

    scanf("%d\n", &n);
    while (n--){
        char s[512];
        int cnt = 0;
        int len;

        fgets(s, 258, stdin);
        len = strlen(s);
        if (s[len - 1] == 10) s[len - 1] = 0;
        for (i = 1; cnt < 26; ++i){
            if (gcd(i, 26) != 1) continue;
            ++cnt;
            for (j = 0; j < 26; ++j){
                char tmp[512];
                int enc[26];
                int dec[26];

                for (k = 0; k < 26; ++k) enc[k] = (k * i + j) % 26;
                for (k = 0; k < 26; ++k) dec[enc[k]] = k;
                strcpy(tmp, s);
                affine_decode(tmp, dec);
                if (is_correct(tmp)){
                    puts(tmp);
                    cnt = 26;
                    j = 26;
                    break;
                }
            }
        }
    }

    return 0;
}