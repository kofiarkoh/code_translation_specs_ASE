#include<stdio.h>
#include<math.h>

typedef struct point{
    double x;
    double y;
}p;


float solve(int n, p a, p b){
    p s, u, t;
    if(n == 0) return 0;

    s.x = (2.0 * a.x + 1.0 * b.x) / 3.0;
    s.y = (2.0 * a.y + 1.0 * b.y) / 3.0;
    t.x = (1.0 * a.x + 2.0 * b.x) / 3.0;
    t.y = (1.0 * a.y + 2.0 * b.y) / 3.0;
    u.x = (t.x - s.x) * cos(M_PI * 1.0 / 3.0) - (t.y - s.y) * sin(M_PI * 1.0 / 3.0) + s.x;
    u.y = (t.x - s.x) * sin(M_PI * 1.0 / 3.0) + (t.y - s.y) * cos(M_PI * 1.0 / 3.0) + s.y;

    solve(n - 1, a, s);
    printf("%.8f %.8f\n", s.x, s.y);
    solve(n - 1, s, u);
    printf("%.8f %.8f\n", u.x, u.y);
    solve(n - 1, u, t);
    printf("%.8f %.8f\n", t.x, t.y);
    solve(n - 1, t, b);
}

int main(void){ 
    int n;
    p a, b;
    a.x = 0;
    a.y = 0;
    b.x = 100;
    b.y = 0;

    scanf("%d", &n);
    printf("%.8f %.8f\n", a.x, a.y);
    solve(n, a, b);
    printf("%.8f %.8f\n", b.x, b.y);
    

}
