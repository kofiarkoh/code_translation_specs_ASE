#include <stdio.h>
#include <string.h>

int main(void)
{
	int color[8][3] = {
		{0, 0, 0},
		{0, 0, 255},
		{0, 255, 0},
		{0, 255, 255},
		{255, 0, 0},
		{255, 0, 255},
		{255, 255, 0},
		{255, 255, 255}
	};
	char color_name[8][8] = {
		"black",
		"blue",
		"lime",
		"aqua",
		"red",
		"fuchsia",
		"yellow",
		"white"
	};
	
	int d[8], R, G, B, min, i;
	char str[8];
	
	while (scanf("%s", str), strcmp(str, "0") != 0){
		min = 0;
		
		if (str[1] >= '0' && str[1] <= '9'){
			R = (str[1] - '0') * 16;
		}
		else {
			R = (str[1] - 'a' + 10) * 16;
		}
		
		if (str[2] >= '0' && str[2] <= '9'){
			R += str[2] - '0';
		}
		else {
			R += str[2] - 'a' + 10;
		}
		
		if (str[3] >= '0' && str[3] <= '9'){
			G = (str[3] - '0') * 16;
		}
		else {
			G = (str[3] - 'a' + 10) * 16;
		}
		if (str[4] >= '0' && str[4] <= '9'){
			G += str[4] - '0';
		}
		else {
			G += str[4] - 'a' + 10;
		}
		
		if (str[5] >= '0' && str[5] <= '9'){
			B = (str[5] - '0') * 16;
		}
		else {
			B = (str[5] - 'a' + 10) * 16;
		}
		if (str[6] >= '0' && str[6] <= '9'){
			B += str[6] - '0';
		}
		else {
			B += str[6] - 'a' + 10;
		}
		
		for (i = 0; i < 8; i++){
			d[i] = (color[i][0] - R) * (color[i][0] - R) 
			+ (color[i][1] - G) * (color[i][1] - G)
			+ (color[i][2] - B) * (color[i][2] - B);
			
			if (d[min] > d[i]){
				min = i;
			}
		}
		
		printf("%s\n", color_name[min]);
	}
	
	return (0);
}