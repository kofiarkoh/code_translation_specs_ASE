#include <stdio.h>
#include <string.h>

int Seisuu(char words[])
{
	int i;
	int kekka;
	i = 0;
	
	kekka = 0;
	
	while (words[i] != '\0'){
		kekka = kekka * 10 + words[i] - '0';
		i++;
	}
	return (kekka);
}

int main(void)
{
	int player;
	int now_playing;
	int now_player[1000];
	int game;
	char words[300];
	int i , j, move;
	int flag;
	
	while (1){
		scanf("%d%d", &player, &game);
		
		if (player == 0 && game == 0){
			break;
		}
		
		memset(now_player, 0, sizeof(now_player));
		
		now_playing = 0;
		
		for (i = 0; i < player; i++){
			now_player[i] = i + 1;
			now_playing++;
			}
		
		move = 0;
		for (i = 0; i < game; i++){
			scanf("%s",words);
			getchar();
			
			flag = 0;
			if (now_playing == 1){
				continue;
			}
			
			if ((i + 1) % 15 == 0){
				if (strcmp(words,"FizzBuzz") != 0){
					for (j = move; j < now_playing - 1; j++) {
						now_player[j] = now_player[j + 1];
					}
					flag = 1;
				}
			}
			
			else if ((i + 1) % 3 == 0){
				if (strcmp(words,"Fizz") != 0){
					for (j = move; j < now_playing - 1; j++) {
						now_player[j] = now_player[j + 1];
					}
					flag = 1;
				}
			}
			
			else if ((i + 1) % 5 == 0){
				if (strcmp(words,"Buzz") != 0){
					for (j = move; j < now_playing - 1; j++) {
						now_player[j] = now_player[j + 1];
					}
					flag = 1;
				}
			}
			
			else {
				if (Seisuu(words) != i + 1){
					for (j = move; j < now_playing - 1; j++) {
						now_player[j] = now_player[j + 1];
					}
					flag = 1;
				}
			}
			if (move < now_playing - 1){
				move += 1;
			}
			else {
				move = 0;
			}
			if (flag == 1){
				now_playing--;
			}
		}
		
		for (i = 0; i < now_playing; i++){
			printf("%d", now_player[i]);
			
			if (i == now_playing - 1){
				break;
			}
			printf(" ");
		}
		printf("\n");
	}
	
	return (0);
}