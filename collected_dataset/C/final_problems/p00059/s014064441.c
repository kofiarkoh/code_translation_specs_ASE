#include <stdio.h>

typedef struct lectangle { double up, down, left, right; } lectangle;
typedef struct point { double x, y; } point;

int isIn(point p, lectangle lec)
{
     return lec.left<=p.x && p.x<=lec.right && lec.up>=p.y && p.y>=lec.down;
}

int main(void)
{
     lectangle lec;
     point p, q, r, s;

     while(~scanf("%lf %lf %lf %lf",
		 &lec.left, &lec.down, &lec.right, &lec.up)){

	  scanf("%lf %lf %lf %lf", &p.x, &p.y, &q.x, &q.y);
	  r.x = q.x, r.y = p.y, s.x = p.x, s.y = q.y;

	  if(isIn(p, lec) || isIn(q, lec) || isIn(r, lec) || isIn(s, lec))
	       puts("YES");
	  else
	       puts("NO");

     }
     
     return 0;
}