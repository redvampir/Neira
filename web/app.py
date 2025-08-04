import streamlit as st

from src.core.neyra_brain import Neyra
from src.interaction import TagProcessor, handle_command

st.title("Нейра в браузере")

if "neyra" not in st.session_state:
    st.session_state.neyra = Neyra()
    st.session_state.processor = TagProcessor()
    st.session_state.history = []

user_input = st.text_input("Введите команду")
if st.button("Отправить") and user_input:
    result = handle_command(st.session_state.neyra, user_input, st.session_state.processor)
    if result.is_exit:
        st.write("Сессия завершена.")
    elif result.text:
        color = {
            "cyan": "cyan",
            "magenta": "magenta",
            "green": "green",
        }.get(result.style or "", "black")
        st.markdown(f"<span style='color:{color}'>{result.text}</span>", unsafe_allow_html=True)
        st.session_state.history.append((user_input, result.text))

if st.session_state.history:
    st.write("## История")
    for command, response in st.session_state.history:
        st.write(f"**> {command}**")
        st.write(response)
