# Planning

This holds some general plans
---


# Splash State

[x] Load -> MenuState


# Menu state

[ ] Select -> GameSetupState


# GameSetupState

- Set up board nothing clickable
- Setup whats clickable for GamePlayState (game dependant)
[ ] Load -> GamePlayState

# GamePlayState

- [ ] Clickable cards -> MovingCardState
- [ ] Drawable deck -> DrawingState
- [ ] Action buttons -> Action Callbacks (game dependant)


# MovingCardState

- [ ] Clickable destinations -> Dropcard Callbacks (game dependant)

# DrawingState

- [ ] Draw to hand -> Ondraw Callbacks (game dependant)
