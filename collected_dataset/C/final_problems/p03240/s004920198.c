#include<stdio.h>

int t[101][101];
char map[101][101];

int absolute(int n){
    return n < 0 ? -1 * n : n;
}

void init(int x, int y, int h){
    for(int i=0; i<=100; i++){
        for(int j=0; j<=100; j++){
            map[i][j] = '.';
            t[i][j] = absolute(i-x) + absolute(j-y) + h;
        }
    }
}

void func(int x, int y, int h){
    for(int i=0; i<=100; i++){
        for(int j=0; j<=100; j++){
            if(map[i][j] == '.'){
                int buf = absolute(i-x) + absolute(j-y) + h;
                if(buf != t[i][j]){
                    map[i][j] = '#';
                }
            }
        }
    }
}

int main(){
    int n;
    scanf("%d", &n);
    for(int i=0; i<n; i++){
        int x, y,h;
        scanf("%d %d %d", &x, &y, &h);
        if(i==0){
            init(x, y, h);
        }else{
            func(x, y, h);
        }
    }
    int ans_x, ans_y, ans_h = -1;
    for(int i=0; i<=100; i++){
        for(int j=0; j<=100; j++){
            if(map[i][j] == '.'){
                if(t[i][j] > ans_h){
                    ans_h = t[i][j];
                    ans_x = i;
                    ans_y = j;
                }
            }
        }
    }
    printf("%d %d %d", ans_x, ans_y, ans_h);

    return 0;
}