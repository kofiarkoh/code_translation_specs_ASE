#include <stdio.h>
#include <math.h>

#define SQ(a) ((a) * (a))

int main(void)
{
	char rgb[8];
	int r, g, b;
	int i;
	double Dk[8];
	double min;
	int num;
	
	scanf("%s", rgb);
	
	while (rgb[0] != '0'){
		
		for (i = 1; i < 7; i++){
			if (rgb[i] <= '9'){
				rgb[i] -= '0';
			}
			else {
				rgb[i] -= 0x57;
			}
		}
		
		r = rgb[1] * 16 + rgb[2];
		g = rgb[3] * 16 + rgb[4];
		b = rgb[5] * 16 + rgb[6];
		
		Dk[0] = sqrt(SQ(r - 0) + SQ(g - 0) + SQ(b - 0));
		Dk[1] = sqrt(SQ(r - 0) + SQ(g - 0) + SQ(b - 255));
		Dk[2] = sqrt(SQ(r - 0) + SQ(g - 255) + SQ(b - 0));
		Dk[3] = sqrt(SQ(r - 0) + SQ(g - 255) + SQ(b - 255));
		Dk[4] = sqrt(SQ(r - 255) + SQ(g - 0) + SQ(b - 0));
		Dk[5] = sqrt(SQ(r - 255) + SQ(g - 0) + SQ(b - 255));
		Dk[6] = sqrt(SQ(r - 255) + SQ(g - 255) + SQ(b - 0));
		Dk[7] = sqrt(SQ(r - 255) + SQ(g - 255) + SQ(b - 255));
		
		num = 0;
		min = 1000;
		for (i = 7; i >= 0; i--){
			if (min >= Dk[i]){
				min = Dk[i];
				num = i;
			}
		}
		
		switch (num){
			case 0:
			printf("black\n");
			break;
			
			case 1:
			printf("blue\n");
			break;
			
			case 2:
			printf("lime\n");
			break;
			
			case 3:
			printf("aqua\n");
			break;
			
			case 4:
			printf("red\n");
			break;
			
			case 5:
			printf("fuchsia\n");
			break;
			
			case 6:
			printf("yellow\n");
			break;
			
			default:
			printf("white\n");
			break;
		}
		
		scanf("%s", rgb);
	}
	
	return (0);
}