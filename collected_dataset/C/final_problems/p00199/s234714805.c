// AOJ Volume 1 Problem 0199 Chairs Where Demanding People Sit

#include <stdio.h>
#include <string.h>

void country_a(char *chair, int n);
void country_b(char *chair, int n);
void country_c(char *chair, int n, int no);
void country_d(char *chair, int n, int no);
                
int main(void)
{
    char chair[101];
    int n, m;
    int i;
    char country[2];
    
    while (1){
        scanf("%d%d", &n, &m);
        if (n == 0 && m == 0){
            break;
        }
        
        memset(chair, '#', n);
        chair[n] = '\0';
        
        if (n == 1){
            scanf("%s", chair);
            printf("%s", chair);
            continue;
        }
        
        for (i = 0; i < m; i++){
            scanf("%s", country);
            switch (country[0]){
              case 'A':
                country_a(chair, n);
                 break;

              case 'B':
                country_b(chair, n);
                 break;

              case 'C':
                country_c(chair, n, i);
                 break;

              case 'D':
                country_d(chair, n, i);
                 break;

            }
        }
        
        printf("%s\n", chair);
    }

    return (0);
}

void country_a(char *chair, int n)
{
    int i;
    
    for (i = 0; i < n; i++){
        if (chair[i] == '#'){
            chair[i] = 'A';
            return;
        }
    }
}

void country_b(char *chair, int n)
{
    int i;
    
    for (i = n - 1; i >= 0; i--){
        if (chair[i] == '#'){
            if (i == n - 1){
                if (chair[i - 1] != 'A'){
                    chair[i] = 'B';
                    return;
                }
            }
            else if (i == 0){
                if (chair[i + 1] != 'A'){
                    chair[i] = 'B';
                    return;
                }
            }
            else {
                if (chair[i - 1] != 'A' && chair[i + 1] != 'A'){
                    chair[i] = 'B';
                    return;
                }
            }
        }
    }
    
    for (i = 0; i < n; i++){
        if (chair[i] == '#'){
            chair[i] = 'B';
            return;
        }
    }
}

void country_c(char *chair, int n, int no)
{
    int i;
    
    if (no == 0){
        if (n % 2 == 0){
            chair[n / 2 + 1] = 'C';
        }
        else {
            chair[(n + 1) / 2] = 'C';
        }
        return;
    }
    
    for (i = 0; i < n; i++){
        if (chair[i] != '#'){
            if (i + 1 < n && chair[i + 1] == '#'){
                chair[i + 1] = 'C';
                return;
            }
            else if (i - 1 >= 0 && chair[i - 1] == '#'){
                chair[i - 1] = 'C';
                return;
            }
        }
    }
}

void country_d(char *chair, int n, int no)
{
    int i;
    int r, l;
    int max_r, max_l;

    if (no == 0){
        chair[0] = 'D';
        return;
    }

    max_r = max_l = -1;
    
    
    l = 0;
    do {
        while (chair[l] != '#'){
            if (l == n - 1){
                l = -1;
                break;
            }
            l++;
        }
        r = l;
        if (r != -1){
            while (chair[r] == '#'){
                if (r == n - 1){
                    break;
                }
                r++;
            }
        }
        
        if (max_r - max_l < r - l){
            max_r = r;
            max_l = l;
        }
        l = r + 1;
    } while (l <= n - 1);


    for (i = 0; i < n; i++){
        if (chair[i] == '#'){
            chair[i] = 'D';
            return;
        }
    }
}